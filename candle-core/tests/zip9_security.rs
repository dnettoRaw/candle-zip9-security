use candle_core::{npy::NpzTensors, pickle, Device, Error, Tensor};

struct Temp(std::path::PathBuf);
impl Temp {
    fn new(name: &str) -> Self {
        Self(std::env::temp_dir().join(format!(
            "dnet-candle-{name}-{}-{:?}.npz",
            std::process::id(),
            std::thread::current().id()
        )))
    }
}
impl Drop for Temp {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

#[test]
fn npz_roundtrip_preserves_unicode_names_and_values() -> candle_core::Result<()> {
    let path = Temp::new("roundtrip");
    let tensor = Tensor::new(&[1f32, 2., 3.], &Device::Cpu)?;
    Tensor::write_npz(&[("échantillon", &tensor)], &path.0)?;
    let loaded = Tensor::read_npz(&path.0)?;
    assert_eq!(loaded[0].0, "échantillon");
    assert_eq!(loaded[0].1.to_vec1::<f32>()?, [1., 2., 3.]);
    let lazy = NpzTensors::new(&path.0)?;
    assert_eq!(lazy.names(), [&"échantillon".to_string()]);
    Ok(())
}

#[test]
fn malformed_archives_propagate_zip_errors() {
    let path = Temp::new("malformed");
    std::fs::write(&path.0, b"PK-invalid-archive").unwrap();
    assert!(matches!(Tensor::read_npz(&path.0), Err(Error::Zip(_))));
    assert!(matches!(NpzTensors::new(&path.0), Err(Error::Zip(_))));
    assert!(matches!(
        pickle::read_pth_tensor_info(&path.0, false, None),
        Err(Error::Zip(_))
    ));
}

#[test]
fn legacy_cp437_filename_remains_readable() -> candle_core::Result<()> {
    let path = Temp::new("cp437");
    let tensor = Tensor::new(&[4f32, 5.], &Device::Cpu)?;
    Tensor::write_npz(&[("x", &tensor)], &path.0)?;
    let mut bytes = std::fs::read(&path.0)?;
    let local = bytes.windows(4).position(|b| b == b"PK\x03\x04").unwrap();
    let central = bytes.windows(4).position(|b| b == b"PK\x01\x02").unwrap();
    for (start, name) in [(local, 30), (central, 46)] {
        bytes[start + name] = 0x82;
    }
    std::fs::write(&path.0, bytes)?;
    let loaded = Tensor::read_npz(&path.0)?;
    assert_eq!(loaded[0].0, "é");
    assert_eq!(loaded[0].1.to_vec1::<f32>()?, [4., 5.]);
    assert_eq!(NpzTensors::new(&path.0)?.names(), [&"é".to_string()]);
    assert!(pickle::read_pth_tensor_info(&path.0, false, None)?.is_empty());
    Ok(())
}
