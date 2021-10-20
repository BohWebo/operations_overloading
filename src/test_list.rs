#[cfg(test)]
mod test_list {
    use crate::list::List;

    #[test]
    fn test_u8 () {
        let list1 = List::from(vec![0_u8, 4_u8, 10_u8, 19_u8, 1_u8, 3_u8, 42_u8]);
        let list2 = List::from(vec![2_u8, 33_u8, 11_u8, 0_u8, 4_u8, 8_u8, 42_u8]);

        let sum  = (&list1 + &list2).into_vec();
        let difference = (&list1 - &list2).into_vec();
        let product = (&list1 * &list2).into_vec();
        let quotient = (&list1 / &list2).into_vec();
        let remainder = (&list1 % &list2).into_vec();

        assert_eq!(sum, vec![2_u8, 37_u8, 21_u8, 19_u8, 5_u8, 11_u8, 84_u8]);
        assert_eq!(difference, vec![0_u8, 0_u8, 0_u8, 19_u8, 0_u8, 0_u8, 0_u8]);
        assert_eq!(product, vec![0_u8, 132_u8, 110_u8, 0_u8, 4_u8, 24_u8, 0_u8]);
        assert_eq!(quotient, vec![0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 1_u8]);
        assert_eq!(remainder, vec![0_u8, 4_u8, 10_u8, 0_u8, 1_u8, 3_u8, 0_u8]);
    }

    #[test]
    fn test_i8 () {
        let list1 = List::from(vec![0_i8, 4_i8, 10_i8, 19_i8, 1_i8, 3_i8, 42_i8, -4_i8, -33_i8]);
        let list2 = List::from(vec![2_i8, 33_i8, 11_i8, 0_i8, 4_i8, 8_i8, 42_i8]);

        let sum  = (&list1 + &list2).into_vec();
        let difference = (&list1 - &list2).into_vec();
        let product = (&list1 * &list2).into_vec();
        let quotient = (&list1 / &list2).into_vec();
        let remainder = (&list1 % &list2).into_vec();

        assert_eq!(sum, vec![2_i8, 37_i8, 21_i8, 19_i8, 5_i8, 11_i8, 84_i8, -2_i8, 0_i8]);
        assert_eq!(difference, vec![-2_i8, -29_i8, -1_i8, 19_i8, -3_i8, -5_i8, 0_i8, -6_i8, -66_i8]);
        assert_eq!(product, vec![0_i8, 0_i8, 110_i8, 0_i8, 4_i8, 24_i8, 0_i8, -8_i8, 0_i8]);
        assert_eq!(quotient, vec![0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 1_i8, -2_i8, -1_i8]);
        assert_eq!(remainder, vec![0_i8, 4_i8, 10_i8, 0_i8, 1_i8, 3_i8, 0_i8, 0_i8, 0_i8]);
    }
}