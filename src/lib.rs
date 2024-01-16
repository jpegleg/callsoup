#[cfg(test)]
mod tests {
    #[test]
    fn uuidtest() {
       use uuid::Uuid;
       assert_eq!(Uuid::new_v4().to_string().is_empty(), false);
    }

    #[test]
    fn datetest() {
       use chrono::prelude::*;
       assert_eq!(Utc::now().to_string().is_empty(), false);
       let dt_nano = NaiveDate::from_ymd_opt(2014, 11, 28).unwrap().and_hms_nano_opt(12, 0, 9, 1).unwrap().and_local_timezone(Utc).unwrap();
       assert_eq!(format!("{:?}", dt_nano), "2014-11-28T12:00:09.000000001Z");
    }

    #[allow(unused_imports)]
    #[test]
    fn blake3() {
      let mut hasher = blake3::Hasher::new();
      hasher.update(b"hello");
      let res = hasher.finalize();
      let hashr = format!("{}", res);
      assert_eq!(hashr, "ea8f163db38682925e4491c5e58d4bb3506ef8c14eb78a86e908c5624a67200f");
    }
}
