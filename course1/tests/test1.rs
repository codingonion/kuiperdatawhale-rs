use nalgebra as na;
use na::Matrix3;

#[test]
fn test_nalgebra_add() {
    let in_matrix1 = Matrix3::new(
        1, 2, 3,
        4, 5, 6,
        7, 8, 9,
    );
    let in_matrix2 = Matrix3::new(
        1, 2, 3,
        4, 5, 6,
        7, 8, 9,
    );
    let out_matrix1 = Matrix3::new(
        2, 4, 6,
        8, 10, 12,
        14, 16, 18,
    );
    let out_matrix2 = in_matrix1 + in_matrix2;
    assert_eq!(out_matrix1, out_matrix2);
}

#[test]
fn test_nalgebra_sub() {
    let in_matrix1 = Matrix3::new(
        1, 2, 3,
        4, 5, 6,
        7, 8, 9,
    );
    let in_matrix2 = Matrix3::new(
        1, 2, 3,
        4, 5, 6,
        7, 8, 9,
    );
    let out_matrix1 = Matrix3::new(
        0, 0, 0,
        0, 0, 0,
        0, 0, 0,
    );
    let out_matrix2 = in_matrix1 - in_matrix2;
    assert_eq!(out_matrix1, out_matrix2);
}

#[test]
fn test_nalgebra_matmul() {
    let in_matrix1 = Matrix3::new(
        1, 2, 3,
        4, 5, 6,
        7, 8, 9,
    );
    let in_matrix2 = Matrix3::new(
        1, 2, 3,
        4, 5, 6,
        7, 8, 9,
    );
    let out_matrix1 = Matrix3::new(
        30,36,42,
        66,81,96,
        102,126,150,
    );
    let out_matrix2 = in_matrix1 * in_matrix2;
    assert_eq!(out_matrix1, out_matrix2);
}

#[test]
fn test_nalgebra_pointwise() {
    let in_matrix1 = Matrix3::new(
        1, 2, 3,
        4, 5, 6,
        7, 8, 9,
    );
    let in_matrix2 = Matrix3::new(
        1, 2, 3,
        4, 5, 6,
        7, 8, 9,
    );
    let out_matrix1 = Matrix3::new(
        1,4,9,
        16,25,36,
        49,64,81,
    );
    let out_matrix2 = in_matrix1.component_mul(&in_matrix2);
    assert_eq!(out_matrix1, out_matrix2);
}