use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum EchoInstruction {
    // Required
    Echo {
        data: Vec<u8>
    },
    // Highly Recommended
    InitializeAuthorizedEcho {
        buffer_seed: u64,
        buffer_size: usize,
    },
    // Highly Recommended
    AuthorizedEcho {
        data: Vec<u8>
    },
    // Optional
    InitializeVendingMachineEcho {
        // Number of tokens required change the buffer
        price: u64,
        buffer_size: usize,
    },
    // Optional
    VendingMachineEcho {
        data: Vec<u8>
    },
}

#[cfg(test)]
mod tests {
    use borsh::{BorshSerialize, BorshDeserialize};
    use super::*;

    #[test]
    fn echo_serde() {
        let data = b"Hello".to_vec();
        let ix = EchoInstruction::Echo { data: data.clone() };
        let expected_ser = vec![
            0, // enum
            5, 0, 0, 0, // length
            b'H', b'e', b'l', b'l', b'o'];
        let ser = ix.try_to_vec().unwrap();
        assert_eq!(expected_ser, ser);
        if let EchoInstruction::Echo { data : got} = ix {
            assert_eq!(&data, &got);
        }
    }

    #[test]
    fn echo_de() {
        let data = EchoInstruction::try_from_slice(
            &[0,
            14, 0, 0, 0,
            72, 101, 108, 108, 111, 44, 32, 83, 111, 108, 97, 110, 97, 33]
        );
        if let EchoInstruction::Echo { data } = data.unwrap() {
            assert_eq!(b"Hello, Solana!".as_ref(), &data);
        }
    }
}
