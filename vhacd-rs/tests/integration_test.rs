#[test]
#[cfg(feature = "ncollide3d")]
fn basic_functionality() {
    use ncollide3d::na::Point3;
    use vhacd::VHACD;

    let _ = env_logger::builder().is_test(true).try_init();
    let cuboid = ncollide3d::procedural::unit_cuboid();
    let params = vhacd::Parameters::default();
    let res = cuboid.vhacd(&params);

    assert_eq!(res.len(), 1);
    let res = res.get(0).unwrap();

    assert_eq!(
        res.coords,
        vec![Point3::new(-0.5, -0.5, -0.5), Point3::new(0.5, 0.5, 0.5),]
    );

    match &res.indices {
        ncollide3d::procedural::IndexBuffer::Unified(indices) => assert_eq!(
            indices,
            &vec![
                Point3::new(4, 3, 7),
                Point3::new(1, 2, 3),
                Point3::new(2, 1, 4),
                Point3::new(1, 3, 4),
            ]
        ),
        _ => panic!(),
    }
}
