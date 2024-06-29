
pub struct IndexMinPQ {
    max_n: i32,
    n: i32,
    pq: Vec<i32>,
    qp: Vec<i32>,
    keys: Vec<f32>
}

impl IndexMinPQ {

    pub fn get_index_from_size(max_n: i32) -> Self {
        //todo добавить исключение
        let mut index = IndexMinPQ {
           max_n,
            n: 0,
            keys: vec![0.0; (max_n + 1) as usize],
            pq: vec![0; (max_n + 1) as usize],
            qp: vec![-1; (max_n + 1) as usize],
        };
        index
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    pub fn size(&self) -> i32 {
        self.n
    }

    pub fn contains(&self, i: usize) -> bool {
        //todo добавить исключение
        self.qp[i] != -1
    }

    pub fn insert(&mut self, i: usize, key: f32) {
        if self.contains(i) {
            //todo добавить исключение
        }
        self.n += 1;
        self.qp[i] = self.n;
        self.pq[self.n as usize] = i as i32;
        self.keys[i] = key;
        Self::swim(self, self.n);
    }

    pub fn del_min(&mut self) -> usize {
        if self.n == 0 {
            //todo добавить исключение
        }
        let min = self.pq[1] as usize;
        self.exch(1usize, self.n as usize);
        self.n -= 1;
        self.sink(1);
        self.qp[min] = -1;
        self.keys[min] = 0.0;
        self.pq[(self.n+1) as usize] = -1;        // not needed
        min
    }

    pub fn change(&mut self, i: usize, key: f32) {
        Self::change_key(self, i, key);
    }

    fn change_key(&mut self, i: usize, key: f32) {
        //todo добавить исключение
        if self.contains(i) {
            //todo добавить исключение
        }
        self.keys[i] = key;
        Self::swim(self, self.qp[i]);
        Self::sink(self, self.qp[i]);
    }

    fn exch(&mut self, i: usize, j: usize) {
        (self.pq[i], self.pq[j])= (self.pq[j], self.pq[i]);
        self.qp[self.pq[i] as usize] = i as i32;
        self.qp[self.pq[j] as usize] = j as i32;
    }

    fn sink(&mut self, mut k: i32) {
        while 2*k <= self.n {
            let mut j = 2*k;
            if j < self.n && Self::greater(self, j, j+1) {
                j += 1;
            }
            if !Self::greater(self, k, j) {
                break;
            }
            self.exch(k as usize, j as usize);
            k = j;
        }
    }

    fn greater(&self, i:i32, j:i32) -> bool {
        let it1 = self.pq[i as usize];
        let it2 = self.pq[j as usize];
        Self::compare(self.keys[it1 as usize], self.keys[it2 as usize])
    }

    fn compare(a: f32, b: f32) -> bool {
        if a > b {
           return  true
        }
        false
    }

    fn swim(&mut self, mut k: i32) {
        while k > 1 && Self::greater(self, k/2, k) {
            Self::exch(self, k as usize, (k/2) as usize);
            k = k/2;
        }
    }
}

