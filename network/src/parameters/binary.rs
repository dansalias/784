use std::{fs, io::{Read, Write, Error}};

use crate::parameters::ParametersFlat;

pub fn read_file(path: &str) -> Result<ParametersFlat, Error> {
    let file = fs::File::create(path)?;

    read(file)
}

pub fn write_file(path: &str, parameters: &ParametersFlat, structure: &[usize]) -> Result<(), Error> {
    let file = fs::File::create(path)?;

    write(file, parameters, structure)
}

fn read_u64<R: Read>(mut reader: R) -> Result<u64, Error> {
    let mut buffer = [0u8; std::mem::size_of::<u64>()];

    reader.read_exact(&mut buffer)?;

    Ok(u64::from_le_bytes(buffer))
}

fn read_f64<R: Read>(mut reader: R) -> Result<f64, Error> {
    let mut buffer = [0u8; std::mem::size_of::<f64>()];

    reader.read_exact(&mut buffer)?;

    Ok(f64::from_le_bytes(buffer))
}

pub fn read<R: Read>(mut reader: R) -> Result<ParametersFlat, Error> {
    let mut structure = Vec::new();

    let number_of_layers = read_u64(&mut reader).unwrap() as usize;

    for _ in 0..number_of_layers {
        structure.push(read_u64(&mut reader).unwrap() as usize);
    }

    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer);

    let parameters = buffer
        .chunks_exact(std::mem::size_of::<f64>())
        .map(|chunk| {
            let mut bytes = [0u8; 8];
            bytes.copy_from_slice(chunk);
            f64::from_le_bytes(bytes)
        })
        .collect();

    Ok(parameters)
    for layer_index in 1..number_of_layers {
        let mut neurons = Vec::new();

        for _ in 0..structure[layer_index] {
            let mut weights = Vec::new();

            for _ in 0..structure[layer_index - 1] {
                weights.push(read_f64(&mut reader).unwrap())
            }

            let bias = read_f64(&mut reader).unwrap();

            neurons.push(( weights, bias ));
        }

        parameters.push(neurons);
    }

    Ok(parameters)
    */
}

pub fn write<W: Write>(mut writer: W, parameters: &ParametersFlat, structure: &[usize]) -> Result<(), Error> {
    [
        structure.len() as u64,
    ]
        .into_iter()
        .chain(
            structure
                .iter()
                .map(|l| *l as u64)
        )
        .map(|n| n.to_le_bytes())
        .chain(
            parameters
                .iter()
                .map(|n| n.to_le_bytes())
        )
        .for_each(|n| {
            let _ = writer.write_all(&n);
        });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn test_parameters() -> ParametersFlat {
        (1..=9).map(|n| n as f64 / 10.0).collect()
    }

    fn test_parameters_encoded() -> Vec<u8> {
        [ 3u64, 2, 2, 1 ]
            .iter()
            .flat_map(|n| n.to_le_bytes())
            .chain(
                [ 0.1f64, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9 ]
                    .iter()
                    .flat_map(|n| n.to_le_bytes())
            )
            .collect::<Vec<u8>>()
    }

    #[test]
    fn reads_parameters() {
        let buffer = Cursor::new(test_parameters_encoded());

        assert_eq!(
            read(buffer).unwrap(),
            test_parameters(),
        );
    }

    #[test]
    fn writes_parameters() {
        let mut buffer = Cursor::new(Vec::new());

        write(&mut buffer, &test_parameters(), &[2, 2, 1]).unwrap();

        assert_eq!(
            buffer.get_ref(),
            &test_parameters_encoded(),
        );
    }
}
