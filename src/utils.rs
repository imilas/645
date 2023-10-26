// pub fn read_file(path: String) -> Result<Vec<i16>, hound::Error> {
//     let mut reader = hound::WavReader::open(path).unwrap();
//     let results: Result<Vec<i16>, _> = reader.samples::<i16>().collect();
//     return results;
// }
pub fn buff_to_vec(path: String) -> Vec<i16> {
    let mut reader = hound::WavReader::open(path).unwrap();
    let mut v = vec![0; reader.len().try_into().unwrap()];
    for (j, i) in reader.samples::<i16>().enumerate() {
        v[j] = i.unwrap();
    }
    return v;
}

pub fn convert_vecs<T, U>(v: Vec<T>) -> Vec<U>
where
    T: Into<U>,
{
    v.into_iter().map(Into::into).collect()
}
