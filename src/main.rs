type Vals = i8;
type Depth = u8;
type Colors = i8;
type Sigs = u64;

const SIZEX: usize = 7;
const SIZEY: usize = 7;

const FOUR: usize = 4;

const MAXDEPTH: Depth = (SIZEX * SIZEY - 1) as Depth;

const EMPTY: Colors = 0;
const WHITE: Colors = 1;
const BLACK: Colors = -WHITE;

//type HVals = [[Sigs; SIZEY]; SIZEX];
type Board = [[Colors; SIZEY]; SIZEX];

const NB_BITS: u8 = 32;
const HASH_SIZE: usize = 1 << NB_BITS;
const HASH_MASK: Sigs = (1 << NB_BITS) - 1;
#[repr(packed)]
#[derive(Clone, Copy, Debug)]
struct HashElem {
    sig: Sigs,
    v_inf: Vals,
    v_sup: Vals,
    d: Depth,
}
const ZHASH: HashElem = HashElem {
    sig: 0,
    v_inf: 0,
    v_sup: 0,
    d: 0,
};
//type HTable2 = Box<[HashElem; HASH_SIZE]>;
type HTable = Vec<HashElem>;

#[allow(dead_code)]
fn eval2(x: usize, y: usize, color: Colors, tab: &Board) -> bool {
    /* Vertical */
    if y >= FOUR - 1 {
        let mut d = 0;
        let nb = loop {
            d += 1;
            let j = y - d;
            if tab[x][j] != color {
                break d - 1;
            }
            if (j == 0) || (d == FOUR - 1) {
                break d;
            }
        };
        if nb >= FOUR - 1 {
	    return true;
        }
    }

    /* Horizontal */
    {
        let mut nb = 0;
        if x < SIZEX - 1 {
            let mut d = 0;
            let res = loop {
                d += 1;
                let i = x + d;
                if tab[i][y] != color {
                    break d - 1;
                }
                if (i == SIZEX - 1) || (d == FOUR - 1) {
                    break d;
                }
            };
            nb += res;
        }
        if x > 0 {
            let mut d = 0;
            let res = loop {
                d += 1;
                let i = x - d;
                if tab[i][y] != color {
                    break d - 1;
                }
                if (i == 0) || (d == FOUR - 1) {
                    break d;
                }
            };
            nb += res;
        }
        if nb >= FOUR - 1 {
	    return true;
        }
    }

    /* Diag 1 */
    {
        let mut nb = 0;
        if (x > 0) && (y > 0) {
            let mut d = 0;
            let res = loop {
                d += 1;
                let i = x - d;
                let j = y - d;
                if tab[i][j] != color {
                    break d - 1;
                }
                if (i == 0) || (j == 0) || (d == FOUR - 1) {
                    break d;
                }
            };
            nb += res;
        }
        if (x < SIZEX - 1) && (y < SIZEY - 1) {
            let mut d = 0;
            let res = loop {
                d += 1;
                let i = x + d;
                let j = y + d;
                if tab[i][j] != color {
                    break d - 1;
                }
                if (i == SIZEX - 1) || (j == SIZEY - 1) || (d == FOUR - 1) {
                    break d;
                }
            };
            nb += res;
        }
        if nb >= FOUR - 1 {
	    return true;
        }
    }

    /* Diag 2 */
    {
        let mut nb = 0;
        if (x > 0) && (y < SIZEY - 1) {
            let mut d = 0;
            let res = loop {
                d += 1;
                let i = x - d;
                let j = y + d;
                if tab[i][j] != color {
                    break d - 1;
                }
                if (i == 0) || (j == SIZEY - 1) || (d == FOUR - 1) {
                    break d;
                }
            };
            nb += res;
        }
        if (x < SIZEX - 1) && (y > 0) {
            let mut d = 0;
            let res = loop {
                d += 1;
                let i = x + d;
                let j = y - d;
                if tab[i][j] != color {
                    break d - 1;
                }
                if (i == SIZEX - 1) || (j == 0) || (d == FOUR - 1) {
                    break d;
                }
            };
            nb += res;
        }
        if nb >= FOUR - 1 {
	    return true;
        }
    }
    false
}

#[allow(dead_code)]
fn eval4(x: usize, y: usize, color: Colors, tab: &Board) -> bool {
    unsafe{
	/* Vertical */
	if y >= FOUR - 1 {
            let mut d = 0;
            let nb = loop {
		d += 1;
		let j = y - d;
		if *tab.get_unchecked(x).get_unchecked(j) != color {break d - 1;}
		if (j == 0) || (d == FOUR - 1) {break d;}
            };
            if nb >= FOUR - 1 {return true;}
	}
	
	/* Horizontal */
	{
            let mut nb = 0;
            if x < SIZEX - 1 {
		let mut d = 0;
		let res = loop {
                    d += 1;
                    let i = x + d;
                    if *tab.get_unchecked(i).get_unchecked(y) != color {break d - 1;}
                    if (i == SIZEX - 1) || (d == FOUR - 1) {break d;}
		};
		nb += res;
            }
            if x > 0 {
		let mut d = 0;
		let res = loop {
                    d += 1;
                    let i = x - d;
                    if *tab.get_unchecked(i).get_unchecked(y) != color {break d - 1;}
                    if (i == 0) || (d == FOUR - 1) {break d;}
		};
		nb += res;
            }
            if nb >= FOUR - 1 {return true;}
	}
	
	/* Diag 1 */
	{
            let mut nb = 0;
            if (x > 0) && (y > 0) {
		let mut d = 0;
		let res = loop {
                    d += 1;
                    let i = x - d;
                    let j = y - d;
                    if *tab.get_unchecked(i).get_unchecked(j) != color {break d - 1;}
                    if (i == 0) || (j == 0) || (d == FOUR - 1) {break d;}
		};
		nb += res;
            }
            if (x < SIZEX - 1) && (y < SIZEY - 1) {
		let mut d = 0;
		let res = loop {
                    d += 1;
                    let i = x + d;
                    let j = y + d;
                    if *tab.get_unchecked(i).get_unchecked(j) != color {break d - 1;}
                    if (i == SIZEX - 1) || (j == SIZEY - 1) || (d == FOUR - 1) {break d;}
		};
		nb += res;
            }
            if nb >= FOUR - 1 {return true;}
	}
	
	/* Diag 2 */
	{
            let mut nb = 0;
            if (x > 0) && (y < SIZEY - 1) {
		let mut d = 0;
		let res = loop {
                    d += 1;
                    let i = x - d;
                    let j = y + d;
                    if *tab.get_unchecked(i).get_unchecked(j) != color {break d - 1;}
                    if (i == 0) || (j == SIZEY - 1) || (d == FOUR - 1) {break d;}
		};
		nb += res;
            }
            if (x < SIZEX - 1) && (y > 0) {
		let mut d = 0;
		let res = loop {
                    d += 1;
                    let i = x + d;
                    let j = y - d;
                    if *tab.get_unchecked(i).get_unchecked(j) != color {break d - 1;}
                    if (i == SIZEX - 1) || (j == 0) || (d == FOUR - 1) {break d;}
		};
		nb += res;
            }
            if nb >= FOUR - 1 {return true;}
	}
	false
    }
}
#[allow(dead_code)]
fn eval3(x: usize, y: usize, color: Colors, tab: &Board) -> bool {
    unsafe{
	/* Vertical */
	if y >= FOUR - 1 {
            let mut d = 0;
            loop {
		d += 1;
		let j = y - d;
		if *tab.get_unchecked(x).get_unchecked(j) != color {break;}
		if d == FOUR - 1 {return true;}
		if j == 0 {break;}
            };
	}
	
	/* Horizontal */
	{
            let mut nb = 0;
            if x < SIZEX - 1 {
		let mut d = 0;
		let res = loop {
                    d += 1;
                    let i = x + d;
                    if *tab.get_unchecked(i).get_unchecked(y) != color {break d - 1;}
		    if d == FOUR - 1 {return true;}
                    if i == SIZEX - 1 {break d;}
		};
		nb += res;
            }
            if x > 0 {
		let mut d = 0;
		loop {
                    d += 1;
                    let i = x - d;
                    if *tab.get_unchecked(i).get_unchecked(y) != color {break;}
		    if d + nb == FOUR - 1 {return true;}
                    if i == 0 {break;}
		};
            }
	}
	
	/* Diag 1 */
	{
            let mut nb = 0;
            if (x > 0) && (y > 0) {
		let mut d = 0;
		let res = loop {
                    d += 1;
                    let i = x - d;
                    let j = y - d;
                    if *tab.get_unchecked(i).get_unchecked(j) != color {break d - 1;}
		    if d == FOUR - 1 {return true;}
                    if (i == 0) || (j == 0) {break d;}
		};
		nb = res;
            }
            if (x < SIZEX - 1) && (y < SIZEY - 1) {
		let mut d = 0;
		loop {
                    d += 1;
                    let i = x + d;
                    let j = y + d;
                    if *tab.get_unchecked(i).get_unchecked(j) != color {break;}
		    if d + nb == FOUR - 1 {return true;}
                    if (i == SIZEX - 1) || (j == SIZEY - 1) {break;}
		};
	    }
	}
	
	/* Diag 2 */
	{
            let mut nb = 0;
            if (x > 0) && (y < SIZEY - 1) {
		let mut d = 0;
		let res = loop {
                    d += 1;
                    let i = x - d;
                    let j = y + d;
                    if *tab.get_unchecked(i).get_unchecked(j) != color {break d - 1;}
                    if d == FOUR - 1 {return true;}
		    if (i == 0) || (j == SIZEY - 1) {break d;}
		};
		nb = res;
            }
            if (x < SIZEX - 1) && (y > 0) {
		let mut d = 0;
		loop {
                    d += 1;
                    let i = x + d;
                    let j = y - d;
                    if *tab.get_unchecked(i).get_unchecked(j) != color {break;}
                    if d + nb == FOUR - 1 {return true;}
		    if (i == SIZEX - 1) || (j == 0) {break;}
		};
            }
	}
	false
    }
}
#[allow(dead_code)]
fn eval(x: usize, y: usize, color: Colors, tab: &Board) -> bool {
    unsafe{
	/* Vertical */
	if y >= FOUR - 1 {
            let mut d = 0;
	    let mut j = y;
            loop {
		d += 1;
		j -= 1;
		if *tab.get_unchecked(x).get_unchecked(j) != color {break;}
		if d == FOUR - 1 {return true;}
		if j == 0 {break;}
            };
	}
	
	/* Horizontal */
	{
            let mut nb = 0;
            if x < SIZEX - 1 {
		let mut d = 0;
		let mut i = x;
		nb = loop {
                    d += 1;
                    i += 1;
                    if *tab.get_unchecked(i).get_unchecked(y) != color {break d - 1;}
		    if d == FOUR - 1 {return true;}
                    if i == SIZEX - 1 {break d;}
		};
            }
            if x > 0 {
		let mut d = 0;
		let mut i = x;
		loop {
                    d += 1;
                    i -= 1;
                    if *tab.get_unchecked(i).get_unchecked(y) != color {break;}
		    if d + nb == FOUR - 1 {return true;}
                    if i == 0 {break;}
		};
            }
	}
	
	/* Diag 1 */
	{
            let mut nb = 0;
            if (x > 0) && (y > 0) {
		let mut d = 0;
		let mut i = x;
		let mut j = y;
		nb = loop {
                    d += 1;
                    i -= 1;
                    j -= 1;
                    if *tab.get_unchecked(i).get_unchecked(j) != color {break d - 1;}
		    if d == FOUR - 1 {return true;}
                    if (i == 0) || (j == 0) {break d;}
		};
            }
            if (x < SIZEX - 1) && (y < SIZEY - 1) {
		let mut d = 0;
		let mut i = x;
		let mut j = y;
		loop {
                    d += 1;
                    i += 1;
                    j += 1;
                    if *tab.get_unchecked(i).get_unchecked(j) != color {break;}
		    if d + nb == FOUR - 1 {return true;}
                    if (i == SIZEX - 1) || (j == SIZEY - 1) {break;}
		};
	    }
	}
	
	/* Diag 2 */
	{
            let mut nb = 0;
            if (x > 0) && (y < SIZEY - 1) {
		let mut d = 0;
		let mut i = x;
		let mut j = y;
		nb = loop {
                    d += 1;
                    i -= 1;
                    j += 1;
                    if *tab.get_unchecked(i).get_unchecked(j) != color {break d - 1;}
                    if d == FOUR - 1 {return true;}
		    if (i == 0) || (j == SIZEY - 1) {break d;}
		};
            }
            if (x < SIZEX - 1) && (y > 0) {
		let mut d = 0;
		let mut i = x;
		let mut j = y;
		loop {
                    d += 1;
                    i += 1;
                    j -= 1;
                    if *tab.get_unchecked(i).get_unchecked(j) != color {break;}
                    if d + nb == FOUR - 1 {return true;}
		    if (i == SIZEX - 1) || (j == 0) {break;}
		};
            }
	}
	false
    }
}

use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
lazy_static! {
    static ref HW:[[Sigs;SIZEY];SIZEX] = {
	let mut rng = thread_rng();
	let mut t = [[0; SIZEY]; SIZEX];
	for item in t.iter_mut() {
	    for item2 in item.iter_mut() {
		*item2 = rng.gen();
	    }
	}
	t
    };
    static ref HB:[[Sigs;SIZEY];SIZEX] = {
	let mut rng = thread_rng();
	let mut t = [[0; SIZEY]; SIZEX];
	for item in t.iter_mut() {
	    for item2 in item.iter_mut() {
		*item2 = rng.gen();
	    }
	}
	t
    };
    static ref FH:Sigs = {
	let mut rng = thread_rng();
	rng.gen()
    };
    static ref IND:[usize;SIZEX]= {
	let mut t = [0;SIZEX];
	for (ix,item) in t.iter_mut().enumerate() {
	    *item=(SIZEX - 1) / 2 + (ix + 1) / 2 * (2 * (ix % 2)) - (ix + 1) / 2;
	}
	t
    };
}



fn retrieve(hv: Sigs, hashes: &HTable) -> Option<(Vals, Vals)> {
    let ind = (hv & HASH_MASK) as usize;
    if hashes[ind].sig == hv {
        Some((hashes[ind].v_inf, hashes[ind].v_sup))
    } else {
        None
    }
}

use core::cmp::{max, min};

fn store(hv: Sigs, alpha: Vals, beta: Vals, g: Vals, depth: Depth, hashes: &mut HTable) {
    let ind = (hv & HASH_MASK) as usize;
    let d = MAXDEPTH + 2 - depth;
    if hashes[ind].d <= d {
        if hashes[ind].sig != hv {
            hashes[ind].d = d;
            hashes[ind].v_inf = Vals::MIN;
            hashes[ind].v_sup = Vals::MAX;
            hashes[ind].sig = hv;
        }
        if (g > alpha) && (g < beta) {
            hashes[ind].v_inf = g;
            hashes[ind].v_sup = g;
        } else if g <= alpha {
            hashes[ind].v_sup = min(g, hashes[ind].v_sup);
        } else if g >= beta {
            hashes[ind].v_inf = max(g, hashes[ind].v_inf);
        }
    }
}

fn ab(
    alpha: Vals,
    beta: Vals,
    color: Colors,
    depth: Depth,
    tab: &mut Board,
    first: &mut [usize; SIZEX],
    hv: Sigs,
    hv2: Sigs,
    hashes: &mut HTable,
) -> Vals {

    //   if hv != compute_hash(color,tab,first_hash,turn_hash,hashesw,hashesb) {panic!("Bad hash");}

    let mut a = alpha;
    let mut b = beta;

    if let Some((v_inf,v_sup)) = retrieve(min(hv, hv2), hashes) {
        if v_inf == v_sup {
            return v_inf;
        }
        if v_inf >= b {
            return v_inf;
        }
        if v_sup <= a {
            return v_sup;
        }
        a = max(a, v_inf);
        b = min(b, v_sup);
    }

    for ix in 0..SIZEX {
	let x = IND[ix];
        let y = first[x];
        if (y != SIZEY) && eval(x, y, color, tab) {return color;}
    }
    if depth == MAXDEPTH {return 0;}
    let mut g= if color == WHITE {Vals::MIN} else {Vals::MAX};
    for ix in 0..SIZEX {
	if a >= b {break;}
	//        let x = (SIZEX - 1) / 2 + (ix + 1) / 2 * (2 * (ix % 2)) - (ix + 1) / 2;
	let x = IND[ix];
        let y = first[x];
        if y < SIZEY {
            tab[x][y] = color;
            first[x] += 1;
            let nhv;
            let nhv2;
            if color == WHITE {
                nhv = hv ^ HW[x][y];
                nhv2 = hv2 ^ HW[SIZEX - 1 - x][y];
            } else {
                nhv = hv ^ HB[x][y];
                nhv2 = hv2 ^ HB[SIZEX - 1 - x][y];
            }
            let v = ab(
                a,
                b,
                -color,
                depth + 1,
                tab,
                first,
                nhv,
		nhv2,
                hashes,
            );
            first[x] -= 1;
            tab[x][y] = EMPTY;
            if color == WHITE {
		g=max(v,g);
		a=max(a,g);
            } else {
		g=min(v,g);
		b=min(b,g);
	    }
        }
    }
    store(min(hv, hv2), alpha, beta, g, depth, hashes);
    g
}

fn compute_hash(tab: &mut Board) -> Sigs {
    let mut h = *FH;
//    if color == BLACK {
//        h ^= turn_hash;
//    }
    for i in 0..SIZEX {
        for j in 0..SIZEY {
            match tab[i][j] {
                BLACK => {
                    h ^= HB[i][j];
                }
                WHITE => {
                    h ^= HW[i][j];
                }
                _ => {}
            }
        }
    }
    h
}

fn main() {
    use std::time::{Instant, SystemTime};
    let mut tab = [[EMPTY; SIZEY]; SIZEX];
    let mut first = [0; SIZEX];
    let mut hashes = vec![ZHASH; HASH_SIZE];
//    let mut hashes = h1.into_boxed_slice();
//    let mut hashes = Box::new([ZHASH; HASH_SIZE]);

    let hv = compute_hash(&mut tab);
    let hv2 = *FH;
    if hv != hv2 {
        panic!("Why???");
    };
    let now = Instant::now();
    let snow = SystemTime::now();
    let ret = ab(
        Vals::MIN,
        Vals::MAX,
        WHITE,
        0,
        &mut tab,
        &mut first,
        hv,
        hv2,
        &mut hashes,
    );
    println!("wall_clock={:?}", now.elapsed());
    println!("system_clock={:?}", snow.elapsed().unwrap());
    println!("ret={}", ret);
}
