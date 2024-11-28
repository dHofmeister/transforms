#[cfg(test)]
mod buffer_tests {
    use crate::types::{Buffer, Duration, Quaternion, Timestamp, Transform, Vector3};

    fn create_transform(t: Timestamp) -> Transform {
        let translation = Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let rotation = Quaternion {
            w: 1.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let timestamp = t;
        let parent = "map".to_string();
        let child = "base".to_string();
        Transform {
            translation,
            rotation,
            timestamp,
            parent,
            child,
        }
    }

    #[test]
    fn insert_and_get() {
        let mut buffer = Buffer::new(Duration::try_from(1.0).unwrap());
        let t = Timestamp::now();
        let transform = create_transform(t);
        buffer.insert(transform.clone());

        let mut r = buffer.get(&transform.timestamp);

        assert!(r.is_ok(), "transform not found");
        assert_eq!(r.unwrap(), transform);

        r = buffer.get(&(transform.timestamp + Duration::try_from(1.0).unwrap()).unwrap());
        assert!(r.is_err(), "transform found, but shouldn't have");

        r = buffer.get(&(transform.timestamp - Duration::try_from(1.0).unwrap()).unwrap());
        assert!(r.is_err(), "transform found, but shouldn't have");
    }

    #[test]
    fn insert_and_get_static() {
        let mut buffer = Buffer::new(Duration::try_from(1.0).unwrap());
        let t = Timestamp::zero();
        let transform = create_transform(t);

        buffer.insert(transform.clone());

        let mut r = buffer.get(&(transform.timestamp + Duration::try_from(1.0).unwrap()).unwrap());

        assert!(r.is_ok(), "transform not found");
        assert_eq!(r.unwrap(), transform);

        r = buffer.get(&(transform.timestamp + Duration::try_from(2.0).unwrap()).unwrap());
        assert!(r.is_ok(), "transform not found");
        assert_eq!(r.unwrap(), transform);
    }

    #[test]
    fn get_nearest() {
        let mut buffer = Buffer::new(Duration::try_from(10.0).unwrap());
        let t = Timestamp::now();

        let p1 = create_transform((t - Duration::try_from(2.0).unwrap()).unwrap());
        let p2 = create_transform((t - Duration::try_from(1.0).unwrap()).unwrap());
        let p3 = create_transform(t);

        buffer.insert(p1.clone());
        buffer.insert(p2.clone());
        buffer.insert(p3.clone());

        // Exact match
        let (before, after) = buffer.get_nearest(&p2.timestamp);
        assert_eq!(before.unwrap(), (&p2.timestamp, &p2));
        assert_eq!(after.unwrap(), (&p2.timestamp, &p2));

        // Between two points
        let p_mid = (p1.timestamp + Duration::try_from(0.5).unwrap()).unwrap();
        let (before, after) = buffer.get_nearest(&p_mid);
        assert_eq!(before.unwrap(), (&p1.timestamp, &p1));
        assert_eq!(after.unwrap(), (&p2.timestamp, &p2));

        // Before first point
        let p_0 = (p1.timestamp - Duration::try_from(1.0).unwrap()).unwrap();
        let (before, after) = buffer.get_nearest(&p_0);
        assert_eq!(before, None);
        assert_eq!(after.unwrap(), (&p1.timestamp, &p1));

        // After last point
        let p_4 = (p3.timestamp + Duration::try_from(1.0).unwrap()).unwrap();
        let (before, after) = buffer.get_nearest(&p_4);
        assert_eq!(before.unwrap(), (&p3.timestamp, &p3));
        assert_eq!(after, None);

        // Exactly at first point
        let (before, after) = buffer.get_nearest(&p1.timestamp);
        assert_eq!(before.unwrap(), (&p1.timestamp, &p1));
        assert_eq!(after.unwrap(), (&p1.timestamp, &p1));

        // Exactly at last point
        let (before, after) = buffer.get_nearest(&p3.timestamp);
        assert_eq!(before.unwrap(), (&p3.timestamp, &p3));
        assert_eq!(after.unwrap(), (&p3.timestamp, &p3));
    }

    #[test]
    fn delete_before() {
        let mut buffer = Buffer::new(Duration::try_from(1.5).unwrap());
        let t = Timestamp::now();

        let p1 = create_transform((t - Duration::try_from(2.0).unwrap()).unwrap());
        let p2 = create_transform((t - Duration::try_from(1.0).unwrap()).unwrap());
        let p3 = create_transform(t);

        buffer.insert(p1.clone());
        buffer.insert(p2.clone());
        buffer.insert(p3.clone());

        let get_1 = buffer.get(&(t - Duration::try_from(2.0).unwrap()).unwrap());
        let get_2 = buffer.get(&(t - Duration::try_from(1.0).unwrap()).unwrap());
        let get_3 = buffer.get(&t);

        assert!(get_1.is_err());
        assert!(get_2.is_ok());
        assert!(get_3.is_ok());
    }

    #[test]
    fn empty_buffer() {
        let buffer = Buffer::new(Duration::try_from(1.0).unwrap());
        assert!(buffer.get(&Timestamp { nanoseconds: 1000 }).is_err());

        let (before, after) = buffer.get_nearest(&Timestamp { nanoseconds: 1000 });
        assert!(before.is_none());
        assert!(after.is_none());
    }

    #[test]
    fn single_point_buffer() {
        let mut buffer = Buffer::new(Duration::try_from(1.0).unwrap());
        let t = Timestamp::now();
        let point = create_transform(t);
        buffer.insert(point.clone());

        // Before the point
        let (before, after) = buffer.get_nearest(&(t - Duration::try_from(1.0).unwrap()).unwrap());
        assert!(before.is_none());
        assert_eq!(after.unwrap(), (&point.timestamp, &point));

        // Exact match
        let (before, after) = buffer.get_nearest(&t);
        assert_eq!(before.unwrap(), (&point.timestamp, &point));
        assert_eq!(after.unwrap(), (&point.timestamp, &point));

        // After the point
        let (before, after) = buffer.get_nearest(&(t + Duration::try_from(1.0).unwrap()).unwrap());
        assert_eq!(before.unwrap(), (&point.timestamp, &point));
        assert!(after.is_none());
    }

    #[test]
    fn delete_expired() {
        let mut buffer = Buffer::new(Duration::try_from(1.0).unwrap());

        let t = Timestamp::now();
        let p1 = create_transform((t - Duration::try_from(2.0).unwrap()).unwrap());
        let p2 = create_transform(t);

        buffer.insert(p1.clone());
        buffer.insert(p2.clone());

        assert!(buffer.get(&p1.timestamp).is_err());
        assert!(buffer.get(&p2.timestamp).is_ok());
    }
}
