use super::types::Point;

#[derive(Debug, Clone)]
pub struct Area {
    pub level: i32,
    pub no: u32,
    pub x_start: i32,
    pub x_end: i32,
    pub y_start: i32,
    pub y_end: i32,
    pub is_in_points: Vec<usize>,
    pub last: bool,
}

#[derive(Debug, Clone)]
pub struct Areas {
    x_length: i32,
    y_length: i32,
    area_list: Vec<Area>,
}


impl Areas {
    pub fn new() -> Self {
        Areas {
            x_length: 0,
            y_length: 0,
            area_list: Vec::new(),
        }
    }

    pub fn create(&mut self, x_start: i32, x_end: i32, y_start: i32, y_end: i32, cur_level: i32, max_level: i32, no: u32, points: &Vec<Point>) {
        self.create_area(x_start, x_end, y_start, y_end, cur_level, max_level, no, points);
    }

    fn create_area(&mut self, x_start: i32, x_end: i32, y_start: i32, y_end: i32, cur_level: i32, max_level: i32, no: u32, points: &Vec<Point>) -> Area {
        let x_mid = ((x_end - x_start) / 2) + x_start;
        let y_mid = ((y_end - y_start) / 2) + y_start;

        if cur_level  != max_level {
            let area0 = self.create_area(x_start, x_mid, y_start, y_mid, cur_level + 1, max_level, 4 * no, points);
            let area1 = self.create_area(x_mid, x_end, y_start, y_mid, cur_level + 1, max_level, 4 * no + 1, points);
            let area2 = self.create_area(x_start, x_mid, y_mid, y_end, cur_level + 1, max_level, 4 * no + 2, points);
            let area3 = self.create_area(x_mid, x_end, y_mid, y_end, cur_level + 1, max_level, 4 * no + 3, points);

            self.area_list.push(area0);
            self.area_list.push(area1);
            self.area_list.push(area2);
            self.area_list.push(area3);

            Area {
                level: cur_level,
                no,
                x_start,
                x_end,
                y_start,
                y_end,
                is_in_points: Vec::new(),
                last: false,
            }

        } else {
            let cells = points.iter()
                .enumerate()
                .filter(|(_i, p)| x_start <= p.x && x_end >= p.x && y_start <= p.y && y_end >= p.y)
                .map(|(i, _p)| i)
                .collect::<Vec<usize>>();

            self.x_length = x_end - x_start;
            self.y_length = y_end - y_start;

            Area {
                level: cur_level,
                no,
                x_start,
                x_end,
                y_start,
                y_end,
                is_in_points: cells,
                last: true,
            }
        }

    }
}

#[derive(Debug, Clone)]
pub struct QuadTree {
    points: Vec<Point>,
    level: i32,
    areas: Areas,
    max_x: i32,
    max_y: i32,
    min_x: i32,
    min_y: i32,
}

impl QuadTree {
    pub fn new(points: Vec<Point>) -> Self {
        //とりあえず64分割
        let max_level = 3;

        //空間の最大値と最小化を取得
        let max_x = points.iter()
            .map(|n| n.x)
            .max()
            .unwrap();

        let max_y = points.iter()
            .map(|n| n.y)
            .max()
            .unwrap();

        let min_x = points.iter()
            .map(|n| n.x)
            .min()
            .unwrap();

        let min_y = points.iter()
            .map(|n| n.y)
            .min()
            .unwrap();

        let mut areas = Areas::new();
        areas.create(min_x, max_x, min_y, max_y, 0, max_level, 0, &points);

        QuadTree {
            points,
            level: max_level,
            areas,
            max_x,
            max_y,
            min_x,
            min_y,
        }
    }

    pub fn convert_point_to_no(&self, p: &Point) -> Option<u32> {
        if self.min_x <=  p.x && self.max_x >= p.x && self.min_y <= p.y && self.max_y >= p.y {
            let x = ((p.x - self.min_x) / self.areas.x_length) as u32;
            let y = ((p.y - self.min_y) / self.areas.y_length) as u32;
            Some(self.bit_separate(x) | self.bit_separate(y) << 1)
        } else {
            None
        }
    }

    fn bit_separate(&self, mut n: u32) -> u32 {
        n = (n | (n << 8)) & 0x00ff00ff;
        n = (n | (n << 4)) & 0x0f0f0f0f;
        n = (n | (n << 2)) & 0x33333333;
        ((n | (n << 1)) & 0x55555555)
    }

    //テスト用 総当たりなので使用しない。
    #[deprecated]
    pub fn get_point(&self, p: &Point) -> Option<u32> {
        let res_vec = self.areas.area_list.iter()
            .filter(|a| a.last && a.x_start <= p.x && a.x_end >= p.x && a.y_start <= p.y && a.y_end >= p.y)
            .map(|a| a.no)
            .collect::<Vec<u32>>();

        if res_vec.len() == 0 {
            None
        } else if res_vec.len() > 1 {
            Some(*res_vec.iter().min().unwrap())
        } else {
            Some(res_vec[0])
        }
    }

    pub fn get_area(&self, no: u32) -> &Area {
        let i = self.areas.area_list.iter().position(|a| a.no == no).unwrap_or(0);
        self.areas.area_list.get(i).unwrap()
    }

    fn get_no(&self, target_no: u32, p1: &Point, p2: &Point, p3: &Point, p4: &Point) -> Vec<u32> {
        let mut no = Vec::new();
        no.push(target_no);

        if let Some(p1) = self.convert_point_to_no(p1) {
            no.push(p1);
        }

        if let Some(p2) = self.convert_point_to_no(p2) {
            no.push(p2);
        }

        if let Some(p3) = self.convert_point_to_no(p3) {
            no.push(p3);
        }

        if let Some(p4) = self.convert_point_to_no(p4) {
            no.push(p4);
        }

        no
    }

    pub fn get_nearest(&self, p: &Point) -> Option<&Point> {
        //与えられた地点の近傍地を計算する。
        let p1 = Point::new(101, "".to_string(), p.x + self.areas.x_length, p.y + self.areas.y_length);
        let p2 = Point::new(101, "".to_string(), p.x - self.areas.x_length, p.y + self.areas.y_length);
        let p3 = Point::new(101, "".to_string(), p.x + self.areas.x_length, p.y - self.areas.y_length);
        let p4 = Point::new(101, "".to_string(), p.x - self.areas.x_length, p.y - self.areas.y_length);

        //指定された地点だけは無かったら終わり。
        let no = {
            let target_p = self.convert_point_to_no(p);
            if target_p.is_none() { return None; }
            target_p.unwrap()
        };

        //ポイントのnoを取得
        let mut no_arr = self.get_no(no, &p1, &p2, &p3, &p4);
        //重複があるので削除
        no_arr.sort();
        no_arr.dedup();

        //別々のvectorからスライスを生成
        let indexes = no_arr.iter()
            .map(|n| self.get_area(*n).is_in_points.as_slice())
            .flat_map(|u| u.iter())
            .map(|n| *n)
            .collect::<Vec<usize>>();

        //距離計算
        if indexes.len() == 1 {
            //近傍1つしかない
            Some(&self.points[indexes[0]])
        } else if indexes.len() > 1 {
            //一番近いのを探す
            let mut min = std::f64::MAX;
            let mut min_index = 0;

            //todo 球を意識した計算しないとダメかな？
            indexes.iter()
                .for_each(|i| {
                    let pnt: &Point = self.points.get(*i).unwrap();
                    let distance = (((pnt.x - p.x) as f64).powi(2) + ((pnt.y - p.y) as f64).powi(2)).sqrt();
                    if min > distance {
                        min = distance;
                        min_index = *i;
                    }
                });
            Some(&self.points[min_index])

        } else {
            //末端の隣接地帯にはないので1つ上のレベルで探す。
            None
        }
    }

}