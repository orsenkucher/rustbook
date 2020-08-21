#[derive(PartialEq, Eq)]
pub enum Sex {
    Male,
    Female,
}

pub struct Scholar {
    name: String,
    age: i32,
    sex: Sex,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_girls_but_not_boys_after() {
        let scholars: Vec<Scholar> = Vec::new();
        let girls_count = scholars
            .into_iter()
            .filter(|s| s.sex == Sex::Female)
            .count();
        assert_eq!(girls_count, 0);
        // let boys_count = scholars.into_iter().filter(|s| s.sex == Sex::Male).count();
    }

    #[test]
    fn filter_girls_and_boys() {
        let scholars: Vec<Scholar> = Vec::new();
        let girls_count = scholars.iter().filter(|s| (**s).sex == Sex::Female).count();
        assert_eq!(girls_count, 0);
        let girls_count = scholars.iter().filter(|s| s.sex == Sex::Female).count();
        assert_eq!(girls_count, 0);
        let boys_count = scholars.iter().filter(|s| s.sex == Sex::Male).count();
        assert_eq!(boys_count, 0);
    }

    #[test]
    fn make_all_same_age() {
        let mut scholars: Vec<Scholar> = Vec::new();
        scholars.iter_mut().for_each(|s| s.age = 18);
    }

    #[test]
    fn basic_filter() {
        let v = vec![1, 2, 3, 4];
        let res: Vec<_> = v.iter().filter(|e| **e % 2 == 0).collect();
        assert_eq!(res, vec![&2, &4]);

        let v = vec![1, 2, 3, 4];
        let res = v.into_iter().filter(|e| *e % 2 == 0).collect::<Vec<_>>();
        assert_eq!(res, vec![2, 4]);
    }
}
