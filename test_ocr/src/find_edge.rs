//trouve les bordure de l'image, afin de les enlever pour ce concentrer uniquement sur le nombre binaire.

pub fn search_edges(matrix: &Vec<Vec<u32>>) -> Option<(usize, usize, usize, usize)> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return None;
    }

    let height = matrix.len();
    let width = matrix[0].len();

    let mut x_min = width;
    let mut x_max = 0;
    let mut y_min = height;
    let mut y_max = 0;
    let mut found_black_pixel = false;

    for y in 0..height {
        for x in 0..width {
            if matrix[y][x] == 0 {
                found_black_pixel = true;
                if x < x_min {
                    x_min = x;
                }
                if x > x_max {
                    x_max = x;
                }
                if y < y_min {
                    y_min = y;
                }
                if y > y_max {
                    y_max = y;
                }
            }
        }
    }

    if !found_black_pixel {
        return None;
    }
    let frame_top = y_min.saturating_sub(1);
    let frame_left = x_min.saturating_sub(1);

    let frame_bottom = if y_max + 1 < height { y_max + 1 } else { y_max };
    let frame_right = if x_max + 1 < width { x_max + 1 } else { x_max };

    Some((frame_top, frame_left, frame_bottom, frame_right))
}

/*fn const_ligne_horizontal(matrix:Vec<Vec<u32>>,x:usize,y:usize,width:usize)->bool{
    for i in x..(width-1) {
        if matrix[y][i] != matrix[y][i+1]{
            return false;
        }
    }
    return true;
}
fn const_ligne_vertical(matrix:Vec<Vec<u32>>,x:usize,y:usize,height:usize)->bool{
    for i in y..(height-1) {
        if matrix[i][x] != matrix[i+1][x]{
            return false;
        }
    }
    return true;
}

fn notfind_bin(matrix:&Vec<Vec<u32>>,x:usize,y:usize,width:usize,height:usize)->bool{
    y < matrix.len() && x < (*matrix[0]).len() && const_ligne_horizontal(matrix.clone(),x,y,width) && const_ligne_vertical(matrix.clone(),x,y,height)
}
fn find_bin(matrix:&Vec<Vec<u32>>,x:usize,y:usize,width:usize,height:usize)->bool{
    y < matrix.len() && x < (*matrix[0]).len() && !const_ligne_horizontal(matrix.clone(),x,y,width) && !const_ligne_vertical(matrix.clone(),x,y,height)
}

pub fn search_edges(matrix:&Vec<Vec<u32>>)->(usize,usize,usize,usize){
    if matrix.len() == 0 {return (0_usize,0_usize,0_usize,0_usize);}
    let mut i:usize = 0;
    let mut j:usize = 0;
    let mut start:bool = false;
    let mut end:bool = false;
    let mut result:(usize,usize,usize,usize) = (0_usize,0_usize,0_usize,0_usize);
    while i <(*matrix).len()-1 && j < (*matrix[0]).len()-1{
        if !start && notfind_bin(matrix,i,j,(*matrix[0]).len() , matrix.len() ) && !notfind_bin(matrix,i+1,j+1,(*matrix[0]).len() , (*matrix).len() ){
            if notfind_bin(matrix, i+1, j, (*matrix[0]).len() , matrix.len() ){
                result.1 = j;
                while i< (*matrix).len()-1 && !start {
                    if !notfind_bin(matrix, i+1, j, (*matrix[0]).len() , matrix.len() ){
                        result.0 = i;
                        start = true;
                    }
                    i+=1;
                }
            }else if notfind_bin(matrix, i, j+1, (*matrix[0]).len() , matrix.len() ){
                result.0 = i;
                while j < (*matrix[0]).len()-1 && !start {
                    if !notfind_bin(matrix, i, j+1, (*matrix[0]).len() , matrix.len() ){
                        result.1 = j;
                        start = true;
                    }
                    j+=1;
                }
            }else{
                result.0 = i;
                result.1 = j;
                start = true;
            }
        }
        if !end && find_bin(matrix,i,j,(*matrix[0]).len() , matrix.len()) && !find_bin(matrix,i+1,j+1,(*matrix[0]).len() , (*matrix).len() ){
            if find_bin(matrix, i+1, j, (*matrix[0]).len() , matrix.len() ){
                result.3 = j+1;
                while i< (*matrix).len()-1 && !end {
                    if !find_bin(matrix, i+1, j, (*matrix[0]).len() , matrix.len() ){
                        result.2 = i+1;
                        end = true;
                    }
                    i+=1;
                }
            }else if find_bin(matrix, i, j+1, (*matrix[0]).len() , matrix.len() ){
                result.2 = i+1;
                while j < (*matrix[0]).len()-1 && !end {
                    if !find_bin(matrix, i, j+1, (*matrix[0]).len() , matrix.len() ){
                        result.3 = j+1;
                        end = true;
                    }
                    j+=1;
                }
            }else{
                result.2 = i+1;
                result.3 = j+1;
                end = true;
            }
        }
        i+=1;
        j+=1;
    }
    if end && start {
        result
    }
    else{
        (0_usize,0_usize,0_usize,0_usize)
    }
}*/
