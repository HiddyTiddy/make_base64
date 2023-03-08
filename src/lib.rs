#[repr(C)]
struct Chunk {
    parts: [u64; 3],
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
    let len_remainder = input.len() % 3;
    let (input, remainder) = input.split_at(input.len() - len_remainder);

    let leftover = input.len() % SIZE_CHUNK;
    let (large, leftover) = input.split_at(input.len() - leftover);

    let large = unsafe {
        std::slice::from_raw_parts(large.as_ptr() as *const Chunk, large.len() / SIZE_CHUNK)
    };

    for elem in large {
        let [a, b, c] = elem.parts.map(u64::from_be);
        let chunk = [
            a >> 58,
            a >> 52,
            a >> 46,
            a >> 40,
            a >> 34,
            a >> 28,
            a >> 22,
            a >> 16,
            a >> 10,
            a >> 4,
            a << 2 | b >> 62,
            b >> 56,
            b >> 50,
            b >> 44,
            b >> 38,
            b >> 32,
            b >> 26,
            b >> 20,
            b >> 14,
            b >> 8,
            b >> 2,
            b << 4 | c >> 60,
            c >> 54,
            c >> 48,
            c >> 42,
            c >> 36,
            c >> 30,
            c >> 24,
            c >> 18,
            c >> 12,
            c >> 6,
            c,
        ]
        .map(|x| (x & 0x3f) as usize)
        .map(|ch| MAP[ch]);

        buf[index..index + chunk.len()].copy_from_slice(&chunk);

        index += chunk.len();

        // ///////
        // for chunk in elem.parts.chunks_exact(3) {
        //     // xxxxxx xxyyyy yyyyzz zzzzzz
        //     let [a, b, c]: [u8; 3] = chunk.try_into().unwrap();
        //
        //     buf[index..index + 4].copy_from_slice(
        //         &[a >> 2, a << 4 | b >> 4, b << 2 | c >> 6, c]
        //             .map(|x| (x & 0x3f) as usize)
        //             .map(|ch| MAP[ch]),
        //     );
        //
        //     index += 4;
        // }
    }

    for chunk in leftover.chunks_exact(3) {
        // xxxxxx xxyyyy yyyyzz zzzzzz
        let [a, b, c]: [u8; 3] = chunk.try_into().unwrap();

        buf[index..index + 4].copy_from_slice(
            &[a >> 2, a << 4 | b >> 4, b << 2 | c >> 6, c]
                .map(|x| (x & 0x3f) as usize)
                .map(|ch| MAP[ch]),
        );

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
        );
    }
}
