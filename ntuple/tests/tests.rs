#[cfg(test)]
mod tests {
    use ntuple::*;

    #[test]
    fn create_and_access_ntuple() {
        let t1 = ntuple!(0, 1, 2, 3, 4, 5);
        let t2 = ntuple!(1, 2, 3, 4, 5, 6);
        assert_eq!(t1[0], 0);
        assert_eq!(t1[1], 1);
        assert_eq!(t1[2], 2);
        assert_eq!(t1[3], 3);
        assert_eq!(t1[4], 4);
        assert_eq!(t1[5], 5);
        assert_ne!(t1, t2);
    }

    #[test]
    fn transform_tuple() {
        let t1 = NTuple::from([0; 4]);
        let t2 = NTuple::from([1; 4]);
        assert_eq!(t1.map(|x| x + 1), t2);

        let t3 = NTuple::from([3; 3]);
        let t4 = NTuple::from([3.0; 3]);
        assert_eq!(t3.map(|x| x as f64), t4);
    }

    #[test]
    fn combine_tuples() {
        let t1 = ntuple!(0, 1, 1, 2, 3, 5);
        let t2 = ntuple!(5, 3, 2, 1, 1, 0);
        let t3 = ntuple!(5, 4, 3, 3, 4, 5);
        assert_eq!(t1.combine(t2, |x, y| x + y), t3);
    }

    #[test]
    fn fold_tuples() {
        let t1 = ntuple!(1, 2, 3);
        assert_eq!(t1.reduce(|acc, x| acc + x), 6);
        assert_eq!(t1.fold(10, |acc, x| acc + x), 16);
    }

    #[test]
    #[should_panic]
    fn reduce_zero_tuple() {
        let zero = NTuple::from([0; 0]);
        zero.reduce(|acc, x| acc + x);
    }

    #[test]
    fn de_serialize() {
        let t = ntuple!(1, 2, 3);
        let s = serde_json::to_string(&t).unwrap();
        let t_de: NTuple<i32, 3> = serde_json::from_str(&s).unwrap();
        assert_eq!(t_de, t);
    }

    #[test]
    fn permute() {
        let t0 = ntuple!(1, 2, 4, 8, 16);
        let p = [4, 3, 2, 1, 0];
        let t1 = t0.permute(p);
        assert_ne!(t0, t1);
        let t2 = t1.permute(p);
        assert_eq!(t0, t2);
    }

    #[test]
    #[should_panic]
    fn bad_permute() {
        let t0 = ntuple!(0);
        let p = [1];
        let t2 = t0.permute(p);
    }
}

