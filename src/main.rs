use image::{DynamicImage,Rgba,GenericImageView};
use cv::feature::akaze::{Akaze, KeyPoint};
use imageproc::drawing;
use rand::Rng;
use tinyfiledialogs;
use tinyfiledialogs::{message_box_ok, MessageBoxIcon, message_box_ok_cancel, OkCancel, open_file_dialog};
use tinyfiledialogs::MessageBoxIcon::{Warning, Info};

fn img_test()
{
    let src_img = image::open(r"C:\Users\Administrator\Pictures\8b82b9014a90f6036d8e4cf63b12b31bb051ed41.jpg").expect("failed open the image!");
    let mut rng = rand::thread_rng();
    let mut canvas = drawing::Blend(src_img.to_rgba());
    for _ in 0..50
    {
        let x:i32 = rng.gen_range(0,src_img.width() -1) as i32;
        let y:i32 = rng.gen_range(0,src_img.height()-1) as i32;
        drawing::draw_cross_mut(&mut canvas,Rgba([0,255,255,128]),x as i32, y as i32);
        let out_img = DynamicImage::ImageRgba8(canvas.0.clone());
        imgshow::imgshow(&out_img);
    }

}
fn img_test2(img_path:String)
{
    let src_image = image::open(img_path).expect("failed to open image file");
    let threshold = 0.001f64;
    let akaze = Akaze::new(threshold);
    let (key_points, _descriptor ) = akaze.extract(&src_image);
    let mut image = drawing::Blend(src_image.to_rgba());
    for KeyPoint{ point:(x,y),..} in key_points
    {
        drawing::draw_cross_mut(&mut image,Rgba([0,0,0,255]),x as i32,y as i32)
    }
    let out_image = DynamicImage::ImageRgba8(image.0.clone());
    imgshow::imgshow(&out_image);

}
fn dialogs_test()
{
    message_box_ok("test","this is a test!",Warning);
    let clicked = message_box_ok_cancel("warning","make a choice",Info,OkCancel::Ok);
    match clicked {
        OkCancel::Ok => println!("you clicked OK"),
        OkCancel::Cancel => println!("you clicked CANCEL"),
    }

}
fn main() {
    println!("Hello, world!");
    let a = ",1";
    let b = a.split(",").collect::<Vec<_>>();
    println!("{:?}",b);
    let a = (1,2,3);
    let (x,y,z) = a;
    println!("{},{},{}",x,y,z);
    let s = "";
    let temp = s.split(",").collect::<Vec<_>>();
    //open a img file
    let img_path = open_file_dialog("open","./",Some((&["jpg"],"*.*")));
    match img_path {
        Some(s) => img_test2(s),
        None => println!("you choose nothing!" ),
    }
    // img_test();
    // img_test2();
}
