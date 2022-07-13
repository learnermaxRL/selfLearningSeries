struct Groups<T> {
    inner: Vec<T>,
    count:usize
}

impl<T> Groups<T> {
    fn new(inner: Vec<T>) -> Self {
	    Groups { inner,count:0 }
    }
}

impl<T: PartialEq> Iterator for Groups<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item>  {
        let vec = Vec::new();
        self.count+=1;
        let curr = &self.inner.get(self.count);
        let loc=0;
        loop {
            self.count+=1;
            if let self.inner[self.count] !=curr {
                vec.insert(loc, curr);
            }  
            loc+=1;
            if self.inner[self.count] != curr {
                break;
            }
        }
        return Some(vec);

    }


    
    
}

fn main() {
    let data = vec![4, 1, 1, 2, 1, 3, 3, -2, -2, -2, 5, 5];
    // groups:     |->|---->|->|->|--->|----------->|--->|
    assert_eq!(
	    Groups::new(data).into_iter().collect::<Vec<Vec<_>>>(),
	    vec![
	        vec![4],
    	    vec![1, 1],
	        vec![2],
    	    vec![1],
	        vec![3, 3],
	        vec![-2, -2, -2],
    	    vec![5, 5],
	    ]
    );

    let data2 = vec![1, 2, 2, 1, 1, 2, 2, 3, 4, 4, 3];
    // groups:      |->|---->|---->|----|->|----->|->|
    assert_eq!(
	    Groups::new(data2).into_iter().collect::<Vec<Vec<_>>>(),
	    vec![
	        vec![1],
    	    vec![2, 2],
	        vec![1, 1],
	        vec![2, 2],
    	    vec![3],
	        vec![4, 4],
	        vec![3],
	    ]
    )
}