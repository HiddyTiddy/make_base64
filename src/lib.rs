pub fn base64<B: AsRef<[u8]>>(input: B, buf: &mut [u8]) -> Option<usize> {
    let input = input.as_ref();
    let mut iter = input.chunks_exact(3);
    const MAP: [u8; 64] = [
        b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O',
        b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd',
        b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's',
        b't', b'u', b'v', b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7',
        b'8', b'9', b'+', b'/',
    ];
    const PADDING: u8 = b'=';

    let mut index = 0;
    for chunk in iter.by_ref() {
        let x = u32::from_le_bytes([chunk[2], chunk[1], chunk[0], 0]);
        let words = [(x >> 18), (x >> 12), (x >> 6), x]
            .map(|x| (x as usize) & 0x3f)
            .map(|i| MAP[i]);
        buf[index..index + 4].copy_from_slice(&words);
        index += 4;
    }

    let remainder = iter.remainder();
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
}
