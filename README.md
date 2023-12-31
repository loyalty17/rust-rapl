# rapl
[![Documentation](https://docs.rs/rapl/badge.svg)](https://docs.rs/rapl)
[![Crate](https://img.shields.io/crates/v/rapl.svg)](https://crates.io/crates/rapl)

Note: `rapl` is in early development and is  not optimized for performance, is not recommended for production applications.

`rapl` is mathematical  computing Rust library that provides a simple way of working with N-dimensional array, along with a wide range of mathematical functions to manipulate them. It takes inspiration from NumPy and APL, with the primary aim of achieving maximum ergonomics and user-friendliness while maintaining generality. 

Our goal is to make Rust scripting as productive as possible, and make Rust a real option when it comes to  numerical computing and data science. Check out the [examples](https://github.com/JErnestoMtz/rapl/tree/main/examples).

Out of the box `rapl` provides features like **co-broadcasting, rank type checking, native complex number support**, among many others:

```Rust
use rapl::*;
fn main() {
    let a = Ndarr::from([1 + 1.i(), 2 + 1.i()]);
    let b = Ndarr::from([[1, 2], [3, 4]]);
    let r = a + b - 2;
    assert_eq!(r, Ndarr::from([[1.i(), 2 + 1.i()],[2 + 1.i(), 4 + 1.i()]]));
}
```

### Array initialization
There are multiple handy ways of initializing N-dimensional arrays (or `Ndarr`).
- From Native Rust arrays to `Ndarr`.
```Rust 
let a = Ndarr::from(["a","b","c"]); 
let b = Ndarr::from([[1,2],[3,4]]);
```
- From ranges.
```Rust
let a = Ndarr::from(1..7).reshape(&[2,3])
```
- From `&str`
```Rust
let chars = Ndarr::from("Hello rapl!"); //Ndarr<char,U1>
```
- Others:
```Rust 
let ones: Ndarr<f32, 2> = Ndarr::ones(&[4,4]);
let zeros : Ndarr<i32, 3>= Ndarr::zeros(&[2,3,4]);
let letter_a = Ndarr::fill("a", &[5]);
let fold = Ndarr::new(data: &[0, 1, 2, 3], shape: [2, 2]).expect("Error initializing");
```
- linspace, logspace, geomspace
```Rust
    let linear = Ndarr::linspace(0, 9, 10);
    assert_eq!(linear,Ndarr::from(0..10));

    let logarithmic = Ndarr::logspace(0.,9., 10., 10);
    assert!(logarithmic.approx(&Ndarr::from([1.,1e1, 1e2, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8, 1e9])));

    let geom = Ndarr::geomspace(1.,256., 9);
    assert!(geom.approx(&Ndarr::from([1., 2., 4., 8., 16., 32., 64., 128., 256.])));

```
### Random array creation
You can easily create random array of any shape:
```Rust
//Normal distribution
let arr_norm = NdarrRand::normal(low: 0f32, high: 1f32, shape: [2, 2], Seed: Some(1234));
//Normal distribution
let arr_uniform = NdarrRand::uniform(low: 0f32, high: 1f32, shape: [10], Seed: None);
//Choose between values
let arr_choose = NdarrRand::choose(&[1, 2, 3, 4, 5], [3, 3], Some(1234));
```

### Element wise operations
- Arithmetic operation with with scalars
```Rust
let ones: Ndarr<i32, 2> = Ndarr::ones(&[4,4]);
let twos = ones + 1;
let sixes = twos * 3;
```
- Arithmetic operation between `Ndarr`s,
```Rust
let a = Ndarr::from([[1,2],[3,4]]);
let b = Ndarr::from([[1,2],[-3,-4]]);

assert_eq!(a + b, Ndarr::from([[2,4],[0,0]]))
```
Note: If the shapes are not equal `rapl` will automatically broadcast the arrays into a compatible shape (if it exist) and perform the operation.
- Math operations including trigonometric and activation functions.
```Rust
let x = Ndarr::from([-1.0 , -0.8, -0.6, -0.4, -0.2, 0.0, 0.2, 0.4, 0.6, 0.8, 1.0]);
let sin_x = x.sin();
let cos_x = x.cos();
let tanh_x = x.tanh();

let abs_x = x.abs();
let relu_x = x.relu();
```
- Map function
```Rust
let a = Ndarr::from([[1,2],[3,4]]);
let mapped = a.map(|x| x*2-1);
```
### Monadic tensor operations
- Transpose
```Rust
let arr = Ndarr::from([[1,2,3],[4,5,6]]);	
assert_eq!(arr.shape(), [2,3]);
assert_eq!(arr.clone().t().shape, [3,2]); //transpose
```
- Reshape
```Rust
let a = Ndarr::from(1..7).reshape(&[2,3]).unwrap();
```
- Slice
```Rust
let arr = Ndarr::from([[1,2],[3,4]]);

assert_eq!(arr.slice_at(1)[0], Ndarr::from([1,3]))
```
- Reduce
```Rust
let sum_axis = arr.clone().reduce(1, |x,y| x + y).unwrap();
assert_eq!(sum_axis, Ndarr::from([6, 15])); //sum reduction
```
- Scan right an left
```Rust
 let s = Ndarr::from([1,2,3]);
 let cumsum = s.scanr( 0, |x,y| x + y);
 assert_eq!(cumsum, Ndarr::from([1,3,6]));
```
- Roll
```Rust
let a = Ndarr::from([[1, 2], [3, 4]]);
assert_eq!(a.roll(1, 1), Ndarr::from([[2, 1], [4, 3]]))
```

### Dyatic tensor operations
- Generalized matrix multiplication between compatible arrays
```Rust
use rapl::*
use rapl::ops::{mat_mul};
let a = Ndarr::from(1..7).reshape(&[2,3]).unwrap();
let b = Ndarr::from(1..7).reshape(&[3,2]).unwrap();
    
let matmul = mat_mul(a, b))
```
- [APL](https://en.wikipedia.org/wiki/APL_(programming_language)) inspired Inner Product.
```Rust
    let a = Ndarr::from(1..7).reshape(&[2,3]).unwrap();
    let b = Ndarr::from(1..7).reshape(&[3,2]).unwrap();
    
    let inner = rapl::ops::inner_product(|x,y| x*y, |x,y| x+y, a.clone(), b.clone());
    assert_eq!(inner, rapl::ops::mat_mul(a, b))

```
- Outer Product.

```Rust
    let suits = Ndarr::from(["♣","♠","♥","♦"]);
    let ranks = Ndarr::from(["2","3","4","5","6","7","8","9","10","J","Q","K","A"]);

    let add_str = |x: &str, y: &str| (x.to_owned() + y);

    let deck = ops::outer_product( add_str, ranks, suits).flatten(); //All cards in a deck
```
### Complex numbers
You can ergonomically do operations between native numeric types and complex types `C<T>` with a simple and clean interface. 
``` Rust
use rapl::*;
// Complex sclars
    let z = 1 + 2.i();
    assert_eq!(z, C(1,2));
    assert_eq!(z - 3, -2 + 2.i());
```

Seamlessly work with complex numbers, and complex tensors.
```Rust
use rapl::*;
// Complex tensors
let arr = Ndarr::from([1, 2, 3]);
let arr_z = arr + -1 + 2.i();
assert_eq!(arr_z, Ndarr::from([C(0,2), C(1,2), C(2,2)]));
assert_eq!(arr_z.im(), Ndarr::from([2,2,2]));
```
### Dead Simple 1D and 2D FFT
```Rust
    let signal = Ndarr::linspace(-10., 10., 100).sin();
    let signal_fft = signal.to_complex().fft();
```

### Image to Array and Array to Image conversion
You can easily work with images of almost any format. `rapl` provides  helpful functions to open images as both RGB and Luma `Ndarr`, and also save them to your preferred format.

```Rust
use rapl::*;
use rapl::utils::rapl_img;

fn main() {
    //open RGB image as  Ndarr<u8,3>
    let img: Ndarr<u8,U3> = rapl_img::open_rgbu8(&"image_name.jpg").unwrap();
    //Split RGB channels by Slicing along 3'th axis.
    let channels: Vec<Ndarr<u8,U2>> = img.slice_at(2);
    //select blue channel and save it as black and white image.
    channels[2].save_as_luma(&"blue_channel.png", rapl_img::ImageFormat::Png);
}
```
### Features in development:
- [x] Port to stable Rust
- [x] Native support for complex numbers.
- [x] Line space and meshigrid initialization.
- [x] Random array creation.
- [x] 1D and 2D FFT.
- [ ] Matrix inversion.
- [x] Image to array conversion.
- [x] Array to image conversion.
- [x] APL-inspired rotate function.
- [x] Commonly use ML functions like Relu, Softmax etc.
- [ ] Support for existing plotting libraries in rust.
- [ ] Mutable slicing.
- [ ] Other Linear algebra functionalities: Eigen, LU, Gauss Jordan, Etc.
- [ ] Automatic differentiation.
