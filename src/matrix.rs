use sprs::{CsMat, CsVec};
use std::cmp::Ordering;
pub struct Matrix {
    users: Vec<u32>,
    items: Vec<u32>,
    values: CsMat<f32>,
}

impl Matrix {
    pub fn new(num_users: usize, num_items: usize) -> Matrix {
        Matrix {
            values: CsMat::zero((num_users, num_items)),
            users: Vec::new(),
            items: Vec::new(),
        }
    }

    pub fn set_user(&mut self, user_id: u32) {
        if self.users.contains(&user_id) {
            return;
        }
        self.users.push(user_id);
    }

    pub fn set_item(&mut self, item_id: u32) {
        if self.items.contains(&item_id) {
            return;
        }
        self.items.push(item_id);
    }

    pub fn set_rating(&mut self, user_id: u32, item_id: u32, rating: f32) {
        if rating == 0.0 {
            return;
        }
        let user_idx = self.users.iter().position(|&x| x == user_id).unwrap();
        let item_idx = self.items.iter().position(|&x| x == item_id).unwrap();
        self.values.insert(user_idx, item_idx, rating);
    }

    fn get_user_vector(&self, user_id: u32) -> CsVec<f32> {
        let user_idx = self.users.iter().position(|&x| x == user_id).unwrap();
        self.values.outer_view(user_idx).unwrap().to_owned()
    }

    fn get_closest_user(&self, user_id: u32) -> Option<u32> {
        let mut closest_user = None;
        let mut closest_distance = -f32::MAX;
        let user_vector = self.get_user_vector(user_id);

        for &user in self.users.iter() {
            if user == user_id {
                continue;
            }
            let other_vector = self.get_user_vector(user);
            let other_distance: f32 = user_vector.dot(&other_vector);
            if other_distance > closest_distance {
                closest_user = Some(user);
                closest_distance = other_distance;
            }
        }
        closest_user
    }

    pub fn get_recommendations(&self, user_id: u32) -> Option<Vec<u32>> {
        let closest_user = self.get_closest_user(user_id)?;
        let closest_vector: CsVec<f32> = self.get_user_vector(closest_user);
        let mut rated_items: Vec<(u32, f32)> = closest_vector
            .iter()
            .filter(|&(_, val)| *val > 0.0)
            .map(|(idx, val)| (self.items[idx], *val))
            .collect();
        rated_items.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
        let recommendations: Vec<u32> = rated_items
            .into_iter()
            .map(|(item_id, _)| item_id)
            .collect();
        if recommendations.is_empty() {
            return None;
        }
        Some(recommendations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let matrix = Matrix::new(3, 3);
        assert_eq!(matrix.users.len(), 0);
        assert_eq!(matrix.items.len(), 0);
        assert_eq!(matrix.values.rows(), 3);
        assert_eq!(matrix.values.cols(), 3);
    }

    #[test]
    fn test_set_user() {
        let mut matrix = Matrix::new(3, 3);
        matrix.set_user(1);
        assert_eq!(matrix.users.len(), 1);
        assert_eq!(matrix.users, vec![1]);
    }

    #[test]
    fn test_set_item() {
        let mut matrix = Matrix::new(3, 3);
        matrix.set_item(1);
        assert_eq!(matrix.items.len(), 1);
        assert_eq!(matrix.items, vec![1]);
    }

    #[test]
    fn test_set_rating() {
        let mut matrix = Matrix::new(3, 3);
        matrix.set_user(1);
        matrix.set_item(1);
        matrix.set_rating(1, 1, 3.14);
        assert_eq!(matrix.values.get(0, 0), Some(&3.14));
    }

    #[test]
    fn test_get_user_vector() {
        let mut matrix = Matrix::new(3, 3);
        matrix.set_user(1);
        matrix.set_item(1);
        matrix.set_rating(1, 1, 3.14);
        let user_vector = matrix.get_user_vector(1);
        assert_eq!(user_vector.nnz(), 1);
        assert_eq!(user_vector.get(0), Some(&3.14));
    }

    #[test]
    fn test_get_closest_user() {
        let mut matrix = Matrix::new(3, 3);
        matrix.set_user(1);
        matrix.set_user(2);
        matrix.set_user(3);
        matrix.set_item(1);
        matrix.set_rating(1, 1, 3.14);
        matrix.set_rating(2, 1, 2.71);
        matrix.set_rating(3, 1, 1.41);
        let closest_user = matrix.get_closest_user(1);
        assert_eq!(closest_user, Some(2));
    }

    #[test]
    fn test_get_recommendations() {
        let mut matrix = Matrix::new(3, 3);
        matrix.set_user(1);
        matrix.set_user(2);
        matrix.set_user(3);
        matrix.set_item(1);
        matrix.set_item(2);
        matrix.set_item(3);
        matrix.set_rating(1, 1, 3.14);
        matrix.set_rating(1, 2, 2.71);
        matrix.set_rating(1, 3, 1.41);
        matrix.set_rating(2, 2, 3.14);
        matrix.set_rating(2, 3, 1.41);
        matrix.set_rating(3, 2, 2.71);
        matrix.set_rating(3, 3, 3.14);
        let recommendations = matrix.get_recommendations(1);
        assert_eq!(recommendations, Some(vec![3, 2]));
    }
}
