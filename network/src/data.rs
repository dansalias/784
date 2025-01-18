use std::fs::File;
use image::GenericImageView;
use parquet::{
    file::reader::{FileReader, SerializedFileReader},
    record::RowAccessor,
};

pub struct DataPoint {
    pub input: Vec<f64>,
    pub expected: Vec<f64>,
}

pub enum Dataset {
    Test,
    Train,
}

pub fn get_data(from: Dataset, limit: Option<usize>) -> Vec<DataPoint> {
    SerializedFileReader::new(
        File::open(
            match from {
                Dataset::Test => "../data/mnist/mnist/test-00000-of-00001.parquet",
                Dataset::Train => "../data/mnist/mnist/train-00000-of-00001.parquet",
            }
        ).unwrap()
    )
        .unwrap()
        .get_row_iter(None)
        .unwrap()
        .take(limit.unwrap_or(usize::MAX))
        .map(|row| {
            let label = row.as_ref().unwrap().get_long(1).unwrap();

            DataPoint {
                input: png_to_pixels(
                    row.as_ref()
                        .unwrap()
                        .get_group(0)
                        .unwrap()
                        .get_bytes(0)
                        .unwrap()
                        .data(),
                ),
                expected: (0..=9)
                    .map(|i| if i == label { 1.0 } else { 0.0 })
                    .collect(),
        }
    })
    .collect::<Vec<DataPoint>>()
}

fn png_to_pixels(png_bytes: &[u8]) -> Vec<f64> {
    image::load_from_memory(png_bytes)
        .unwrap()
        .grayscale()
        .pixels()
        .map(|(_, _, rgba)| {
            rgba.0[0] as f64 / 255.0
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_data() {
        let data = get_data(Dataset::Test, Some(5));

        assert_eq!(data.len(), 5);

        assert_eq!(data[0].input.len(), 784);
    }
}
