
#[derive(Debug)]
pub struct GraphList {
    pub def:Vec<Vec<usize>>,
    pub nb_arrows:usize,
}
#[derive(Debug)]
pub struct GraphMat {
    pub def:Vec<Vec<bool>>,
    pub nb_arrows:usize,
}


pub trait Graph {
    fn get_nb_arrows(&self) -> usize;
    fn size(&self) -> usize;
    fn get_positive_degree(&self,vertex:usize) -> Vec<usize>;
    fn dfs(&self) -> (Vec<usize>,Vec<usize>);
    fn dfs_single(&self, v: usize, state: &mut Vec<bool>, parent: &mut Vec<usize>, prefixe: &mut Vec<usize>, suffixe: &mut Vec<usize>,p: &mut usize,s: &mut usize) -> ();
    fn detect_cycle(&self,prefixe: &mut Vec<usize>,suffixe: &mut Vec<usize>) -> (Vec<Vec<usize>>,u32);
    fn preprocess_graphe(&mut self) -> Vec<usize>;
    fn add_arrow(&mut self,i:usize,word:usize) -> ();
    fn actualise(&mut self, solution_pro:Vec<usize>);
}

impl Graph for GraphList {

    fn get_nb_arrows(&self) -> usize {
        self.nb_arrows
    }

    fn add_arrow(&mut self,i:usize,word:usize) -> () {
        self.def[i].push(word);
    }
    fn size(&self) -> usize {
        self.def.len()
    }
    fn get_positive_degree(&self,vertex:usize) -> Vec<usize> {
        let mut pos_degree = vec![];
        for elem in self.def[vertex].iter().collect::<Vec<&usize>>() {
            pos_degree.push(*elem); 
        }
        pos_degree
    }
    fn dfs(&self) -> (Vec<usize>,Vec<usize>) {
        let mut state = vec!(false;self.size());
        let mut parent = vec!(0;self.size());
        let mut prefixe = vec!(0;self.size());
        let mut suffixe = vec!(0;self.size());
        let mut p:usize = 1;
        let mut s:usize = 1;
        for i in 0..self.size() {
            if state[i] == false {
                Self::dfs_single(self, i,&mut state,&mut parent,&mut prefixe,&mut suffixe,&mut p,&mut s);
            }   
        }
        (prefixe,suffixe)
    }
    fn dfs_single(&self, v: usize, state: &mut Vec<bool>, parent: &mut Vec<usize>, prefixe: &mut Vec<usize>, suffixe: &mut Vec<usize>,p: &mut usize,s: &mut usize) -> () {
        state[v] = true;
        prefixe[v] = *p;
        *p = *p+1;
        for j in self.get_positive_degree(v) {
            if state[j] == false {
                parent[j] = v+1;
                Self::dfs_single(self,j,state,parent,prefixe,suffixe,p,s);
            }
        }
        suffixe[v] = *s;
        *s = *s+1;
    }
    fn detect_cycle(&self,prefixe: &mut Vec<usize>,suffixe: &mut Vec<usize>) -> (Vec<Vec<usize>>,u32) {
        let mut arrows = vec![vec![];prefixe.len()];
        let mut nb:u32 = 0;
        for i in 0..prefixe.len() {
            for j in 0..prefixe.len() {
                if ((prefixe[i] > prefixe[j]) && (suffixe[i] < suffixe[j])) && (self.def[i].contains(&j)) {
                    arrows[i].push(j);
                    nb+=1;
                }
            }
        }
        (arrows,nb)
    }
    fn preprocess_graphe(&mut self) -> Vec<usize> {
        vec![]
    }

    fn actualise(&mut self, solution_pro:Vec<usize>) {
        // mind vertex+1 in solution
        /*for elem in solution_pro {
            for i in 0..self.def.len() {
                let test = self.def[i].position(|&x| x == (elem-1)); 
                self.def[i].remove((elem-1));
            }
            self.def[elem-1] = vec![];
        }*/
        for &elem in &solution_pro {
            for def in &mut self.def {
                let index = def.iter().position(|&x| x == elem - 1);
                if let Some(i) = index {
                    def.remove(i);
                }
            }
            self.def[elem - 1] = vec![];
        }
    }
}

impl Graph for GraphMat {

    fn get_nb_arrows(&self) -> usize {
        self.nb_arrows
    }

    fn add_arrow(&mut self,i:usize,word:usize) -> () {
        self.def[i][word] = true;
    }

    fn size(&self) -> usize {
        self.def.len()
    }

    fn get_positive_degree(&self,vertex:usize) -> Vec<usize> {
        let mut pos_degree = vec![];
        for j in 0..self.size() {
            if self.def[vertex][j] == true {
                pos_degree.push(j);
            }
        }
        pos_degree
    }

    /// returns the prefixe and suffixe of vertex in 2 vec (first is prefixe, second is suffixe)
    fn dfs(&self) -> (Vec<usize>,Vec<usize>) {
        let mut state = vec!(false;self.size());
        let mut parent = vec!(0;self.size());
        let mut prefixe = vec!(0;self.size());
        let mut suffixe = vec!(0;self.size());
        let mut p:usize = 1;
        let mut s:usize = 1;
        for i in 0..self.size() {
            if state[i] == false {
                Self::dfs_single(self, i,&mut state,&mut parent,&mut prefixe,&mut suffixe,&mut p,&mut s);
            }   
        }
        (prefixe,suffixe)
    }

    fn dfs_single(&self, v: usize, state: &mut Vec<bool>, parent: &mut Vec<usize>, prefixe: &mut Vec<usize>, suffixe: &mut Vec<usize>,p: &mut usize,s: &mut usize) -> () {
        state[v] = true;
        prefixe[v] = *p;
        *p = *p+1;
        for j in self.get_positive_degree(v) {
            if state[j] == false {
                parent[j] = v+1;
                Self::dfs_single(self,j,state,parent,prefixe,suffixe,p,s);
            }
        }
        suffixe[v] = *s;
        *s = *s+1;
    }

    /// returns the number of cycles and the back_arrows which create a cycle
    fn detect_cycle(&self,prefixe: &mut Vec<usize>,suffixe: &mut Vec<usize>) -> (Vec<Vec<usize>>,u32) {
        let mut arrows = vec![vec![];prefixe.len()];
        let mut nb:u32 = 0;
        for i in 0..prefixe.len() {
            for j in 0..prefixe.len() {
                if ((prefixe[i] > prefixe[j]) && (suffixe[i] < suffixe[j])) && (self.def[i][j] == true){
                    arrows[i].push(j);
                    nb+=1;
                }
            }
        }
        (arrows,nb)
    }

    /// mini preprocess, taking care of the looping vertex -- to upgrade
    /// returns the vertex which are "looping vertex", and modify as well the original graphe to make it like the vertex is removed
    fn preprocess_graphe(&mut self) -> Vec<usize> {
        let mut solution: Vec<usize> = vec![];
        for i in 0..self.size() {
            if self.def[i][i] == true {
                solution.push(i+1);
                self.def[i][i] = false;
                for y in 0..self.size() {
                    self.def[y][i] = false;
                }
            }
        }
        solution
    }

    fn actualise(&mut self, solution_pro:Vec<usize>) {
        // attention elem est incrémenter de 1 c'est le sommet
        for elem in solution_pro {
            for i in 0..self.def.len() {
                self.def[elem-1][i] = false;
                self.def[i][elem-1] = false;
            }
        }
    }

}

/// takes an array of back_arrows and returns the occurence of back_arrows which create cycle + totalNumber of occurence for optimiser_retirerV2
pub fn count_occurence_and_give_tuple(back_arrows: &mut Vec<Vec<usize>>) -> (Vec<usize>,Vec<(usize,usize)>,usize) {
    let mut total_occurence:usize = 0;
    let mut nb_occurence = vec!(0;back_arrows.len());
    let mut back_arrows_tuple : Vec<(usize,usize)>= vec![];
    for i in 0..back_arrows.len() {
        for elem in back_arrows[i].iter() {
            if back_arrows[i].contains(elem) {
                nb_occurence[*elem] += 1;
                nb_occurence[i] += 1;
                back_arrows_tuple.push((i+1,*elem+1));
                total_occurence += 2;
            }
        }
    }
    (nb_occurence,back_arrows_tuple,total_occurence)
}

/// this function takes the back_arrows of a graph and returns the vertex to remove in order to make it acyclic, in a optimal way
pub fn optimise_retirer(back_arrows: &mut Vec<Vec<usize>>) -> Vec<usize> {
    let mut solutions : Vec<usize> = vec![];
    let (mut nb_occurence,mut back_arrows_tuple,_) = count_occurence_and_give_tuple(back_arrows);

    let (mut i_max,mut max) = match nb_occurence.iter().enumerate().max_by_key(|&(_,item)| item) {
        Some((index, number)) => (index,*number),
        None => panic!("error : back_arrows vide in optimise_retirer"),
    };

    while max != 0 && back_arrows_tuple.len() > 0 {
        solutions.push(i_max+1);

        back_arrows_tuple.retain(|(x, y)| {
            if *x == i_max+1 || *y == i_max+1 {
                if *x == i_max+1 {
                    nb_occurence[*y-1] -= 1;
                } else if *y == i_max+1 {
                    nb_occurence[*x-1] -= 1;
                }
                false
            } else {
                true
            }
        });
        nb_occurence[i_max] = 0;
        (i_max,max) = match nb_occurence.iter().enumerate().max_by_key(|&(_,item)| item) {
            Some((index, number)) => (index,*number),
            None => panic!("error : back_arrows vide in optimise_retirer"),
        };
    }
    solutions
}

/// this function takes the back_arrows of a graph and returns the vertex to remove in order to make it acyclic, in a optimal way
pub fn optimise_retirer_opti(back_arrows: &mut Vec<Vec<usize>>) -> (Vec<usize>,usize) {
    let mut solutions : Vec<usize> = vec![];
    let (mut nb_occurence,mut back_arrows_tuple,mut total_occurence) = count_occurence_and_give_tuple(back_arrows);
    let fix_total_occurence = total_occurence;

    let (mut i_max,mut max) = match nb_occurence.iter().enumerate().max_by_key(|&(_,item)| item) {
        Some((index, number)) => (index,*number),
        None => panic!("error : back_arrows vide in optimise_retirer"),
    };

    while (max != 0 && back_arrows_tuple.len() > 0) && (total_occurence as f32 > fix_total_occurence as f32/2.0)  {
        solutions.push(i_max+1);

        back_arrows_tuple.retain(|(x, y)| {
            if *x == i_max+1 || *y == i_max+1 {
                if *x == i_max+1 {
                    nb_occurence[*y-1] -= 1;
                    total_occurence -= 1;
                } else if *y == i_max+1 {
                    nb_occurence[*x-1] -= 1;
                    total_occurence -= 1;
                }
                false
            } else {
                true
            }
        });
        total_occurence -= nb_occurence[i_max];
        nb_occurence[i_max] = 0;
        (i_max,max) = match nb_occurence.iter().enumerate().max_by_key(|&(_,item)| item) {
            Some((index, number)) => (index,*number),
            None => panic!("error : back_arrows vide in optimise_retirer"),
        };
    }
    (solutions,total_occurence)
}