#[repr(C)]
struct Chunk {
    // parts: [u64; 3],
    parts: [u8; 3],
}

#[inline]
pub fn length_required(input_len: usize) -> usize {
    ((input_len + 2) / 3) * 4
}

pub fn base64<B: AsRef<[u8]>>(input: B, buf: &mut [u8]) -> Option<usize> {
    let input = input.as_ref();
    if length_required(input.len()) > buf.len() {
        return None;
    }
    const MAP: [u8; 64] = [
        b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O',
        b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd',
        b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's',
        b't', b'u', b'v', b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7',
        b'8', b'9', b'+', b'/',
    ];
    const PADDING: u8 = b'=';

    const ALIGN_CHUNK: usize = std::mem::align_of::<Chunk>();
    const SIZE_CHUNK: usize = std::mem::size_of::<Chunk>();
    let mut i = 0;
    let repeat_until_aligned =
        ((ALIGN_CHUNK - (input.as_ptr() as usize % (ALIGN_CHUNK))) * 3) % ALIGN_CHUNK;

    // the number of full triplets
    let num_triplets = input.len() / 3;
    // println!(
    //     "{:?} {} {repeat_until_aligned}",
    //     input.as_ptr(),
    //     input.as_ptr() as usize % ALIGN_CHUNK
    // );
    // println!("{num_triplets}");

    let mut index = 0;
    let its = std::cmp::min(num_triplets, repeat_until_aligned);
    for _ in 0..its {
        // xxxxxx xxyyyy yyyyzz zzzzzz
        let [a, b, c]: [u8; 3] = input[i..i + 3].try_into().unwrap();

        buf[index..index + 4].copy_from_slice(
            &[a >> 2, a << 4 | b >> 4, b << 2 | c >> 6, c]
                .map(|x| (x & 0x3f) as usize)
                .map(|ch| MAP[ch]),
        );

        i += 3;
        index += 4;
    }

    let input = &input[i..];
    // println!(
    //     "{:?} {}",
    //     input.as_ptr(),
    //     input.as_ptr() as usize % ALIGN_CHUNK
    // );
    let len_remainder = input.len() % 3;
    let (input, remainder) = input.split_at(input.len() - len_remainder);

    let leftover = input.len() % SIZE_CHUNK;
    let (large, leftover) = input.split_at(input.len() - leftover);
    // println!("{}", large.len());

    let large = unsafe {
        std::slice::from_raw_parts(large.as_ptr() as *const Chunk, large.len() / SIZE_CHUNK)
    };

    for elem in large {
        // let chunk = dbg!([
        //     elem.parts[0] >> 58,
        //     elem.parts[0] >> 52,
        //     elem.parts[0] >> 46,
        //     elem.parts[0] >> 40,
        //     elem.parts[0] >> 34,
        //     elem.parts[0] >> 28,
        //     elem.parts[0] >> 22,
        //     elem.parts[0] >> 16,
        //     elem.parts[0] >> 10,
        //     elem.parts[0] >> 4,
        //     elem.parts[0] << 2 | elem.parts[1] >> 62,
        //     elem.parts[1] >> 56,
        //     elem.parts[1] >> 50,
        //     elem.parts[1] >> 44,
        //     elem.parts[1] >> 38,
        //     elem.parts[1] >> 32,
        //     elem.parts[1] >> 26,
        //     elem.parts[1] >> 20,
        //     elem.parts[1] >> 14,
        //     elem.parts[1] >> 8,
        //     elem.parts[1] >> 2,
        //     elem.parts[1] << 4 | elem.parts[2] >> 60,
        //     elem.parts[2] >> 54,
        //     elem.parts[2] >> 48,
        //     elem.parts[2] >> 42,
        //     elem.parts[2] >> 36,
        //     elem.parts[2] >> 30,
        //     elem.parts[2] >> 24,
        //     elem.parts[2] >> 18,
        //     elem.parts[2] >> 12,
        //     elem.parts[2] >> 6,
        //     elem.parts[2],
        // ]
        // .map(|x| (x & 0x3f) as usize))
        // .map(|ch| MAP[ch]);
        //
        // buf[index..index + chunk.len()].copy_from_slice(&chunk);

        // index += chunk.len();
        for chunk in elem.parts.chunks_exact(3) {
            // xxxxxx xxyyyy yyyyzz zzzzzz
            let [a, b, c]: [u8; 3] = chunk.try_into().unwrap();

            buf[index..index + 4].copy_from_slice(
                &[a >> 2, a << 4 | b >> 4, b << 2 | c >> 6, c]
                    .map(|x| (x & 0x3f) as usize)
                    .map(|ch| MAP[ch]),
            );

            i += 3;
            index += 4;
        }
    }

    // while i + 3 < input.len() {
    //     let _ = input[i..i + 4];
    //     let x = u32::from_le_bytes([chunk[2], chunk[1], chunk[0], 0]);
    //     let words = [(x >> 18), (x >> 12), (x >> 6), x]
    //         .map(|x| (x as usize) & 0x3f)
    //         .map(|i| MAP[i]);
    //     buf[index..index + 4].copy_from_slice(&words);
    //     index += 4;
    //     i += 3;
    // }

    for chunk in leftover.chunks_exact(3) {
        // xxxxxx xxyyyy yyyyzz zzzzzz
        let [a, b, c]: [u8; 3] = chunk.try_into().unwrap();

        buf[index..index + 4].copy_from_slice(
            &[a >> 2, a << 4 | b >> 4, b << 2 | c >> 6, c]
                .map(|x| (x & 0x3f) as usize)
                .map(|ch| MAP[ch]),
        );

        i += 3;
        index += 4;
    }

    // let remainder = &input[input.len() - len_remainder..];
    match remainder.len() {
        1 => {
            // xxxxxx xx0000
            let words = [remainder[0] >> 2, remainder[0] << 4]
                .map(|x| (x as usize) & 0x3f)
                .map(|ch| MAP[ch]);
            buf[index..index + 4].copy_from_slice(&[words[0], words[1], PADDING, PADDING]);

            index += 4;
        }
        2 => {
            // xxxxxx xxyyyy yyyy00
            let words = [
                remainder[0] >> 2,
                (remainder[0] << 4) | remainder[1] >> 4,
                remainder[1] << 2,
            ]
            .map(|x| (x as usize) & 0x3f)
            .map(|ch| MAP[ch]);
            buf[index..index + 4].copy_from_slice(&[words[0], words[1], words[2], PADDING]);
            index += 4;
        }
        _ => {}
    }

    Some(index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unpadded() {
        let input = "hello!";
        let mut buffer = [0; 12];
        let read = base64(input, &mut buffer).expect("enough space");
        assert_eq!(&buffer[..read], "aGVsbG8h".as_bytes());
    }

    #[test]
    fn padded() {
        let input = "hello world";
        let mut buffer = [0; 64];
        let read = base64(input, &mut buffer).expect("enough space");
        assert_eq!(&buffer[..read], "aGVsbG8gd29ybGQ=".as_bytes());
    }

    #[test]
    fn longer() {
        let input = "some fairly long string of text to be converted into base64";
        assert_eq!(length_required(input.len()), 80);
        let mut buffer = [0; 128];
        let read = base64(input, &mut buffer).expect("enough space");
        assert_eq!(
            &buffer[..read],
            "c29tZSBmYWlybHkgbG9uZyBzdHJpbmcgb2YgdGV4dCB0byBiZSBjb252ZXJ0ZWQgaW50byBiYXNlNjQ="
                .as_bytes()
        )
    }
}
