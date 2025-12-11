fn make_globe_mesh() {
    let image_bytes = include_bytes!("../World_elevation_map.png");
    let img = image::load_from_memory(image_bytes).unwrap().into_luma8();
    let scale = 10;
    let width = img.width() as usize / scale;
    let height = img.height() as usize / scale;
    let mut elevs = Vec::with_capacity(width * height);
    for i in 0..width {
        let x = (i * scale) as u32;
        for j in 0..height {
            let y = (j * scale) as u32;
            let px = img.get_pixel(x, y).0[0];
            elevs.push(px)
        }
    }
    //TO DO::: CREATE VERTICIES
}
