use std::{fs};
use std::collections::{HashMap, HashSet};
use std::ops;

fn main() {
    let (enhancements, image) = get_scanners();
    let nimage = increase_frame(&image, false, 5);
    print_image(&nimage);
    let val = get_pixel_value(&nimage, 7, 7);
    println!("{}", val);
}

fn print_image(image: &Vec<Vec<bool>>){
    for row in image{
        let buff: String = row.into_iter().map(|x| if !x {'.'} else {'#'}).collect();
        println!("{}", buff);
    }
}

fn get_pixel_value(image: &Vec<Vec<bool>>, row: usize, col: usize) -> u32{
    let nrows = image.len();
    let ncols = image[0].len();
    let mut num = 0;
    let mut offset = 8;
    for i_row in row-1..row+2{
        for i_col in col-1..col+2{
            if image[i_row][i_col]{
                num |= 1 << offset;
            }
            offset -= 1;
        }
    }
    num
}

fn increase_frame(image: &Vec<Vec<bool>>, value: bool, size: usize) -> Vec<Vec<bool>>{
    let nrows = image.len();
    let ncols = image[0].len();
    let mut new_image: Vec<Vec<bool>> = Vec::new();
    let new_row = vec![value; ncols + (size * 2)];
    for _ in 0..size{
        new_image.push(new_row.clone());
    }
    for row in image{
        let mut buff = vec![value; size*2];
        buff.splice(size..size, row.clone());
        new_image.push(buff);
    }
    for _ in 0..size{
        new_image.push(new_row.clone());
    }
    new_image
}

fn get_scanners() -> (Vec<bool>, Vec<Vec<bool>>){
    let filename = "./data/day20.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file").replace("\r", "");
    let lines: Vec<String> = contents.split("\n").map(str::to_string).collect();
    let mut image: Vec<Vec<bool>> = Vec::new();
    let enhancement: Vec<bool> = lines[0].chars().into_iter().map(|x| x=='#').collect();
    for idx in 1..lines.len(){
        let buff: Vec<bool> = lines[idx].chars().into_iter().map(|x| x=='#').collect();
        if buff.len() > 0 {image.push(buff)}
    }
    (enhancement, image)
}