// Nokta yapısı: x ve y koordinatlarını f64 olarak saklar.
struct Point {
    x: f64,
    y: f64,
}

// Point yapısı için impl blokları birleştirildi.
impl Point {
    // Köken noktası (0.0, 0.0) döndüren associated function.
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Verilen x ve y değerleriyle yeni bir Point oluşturur.
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

// Dikdörtgen yapısı: İki nokta (p1 ve p2) ile tanımlanır.
struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // Dikdörtgenin alanını hesaplar: |x1 - x2| * |y1 - y2|
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        (x1 - x2).abs() * (y1 - y2).abs()
    }

    // Dikdörtgenin çevresini hesaplar: 2 * (|x1 - x2| + |y1 - y2|)
    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // Dikdörtgeni verilen x ve y değerleri kadar ötele.
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;
        self.p1.y += y;
        self.p2.y += y;
    }
}

// İki Box<i32> içeren bir tuple struct.
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // Pair'i yok eder ve değerlerini ekrana basar.
    fn destroy(self) {
        let Pair(first, second) = self;
        println!("Destroying Pair({}, {})", first, second);
    }
}

// Örnek kullanım fonksiyonu.
pub fn execute_method() {
    // Köken noktası ve (3.0, 4.0) noktasıyla bir dikdörtgen oluştur.
    let mut rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Alan ve çevreyi hesapla ve ekrana bas.
    println!("Area: {:?}", rectangle.area());
    println!("Perimeter: {:?}", rectangle.perimeter());

    // Dikdörtgeni (1.0, 1.0) kadar ötele.
    rectangle.translate(1.0, 1.0);

    // Yeni alan ve çevreyi ekrana bas.
    println!("New Area: {:?}", rectangle.area());
    println!("New Perimeter: {:?}", rectangle.perimeter());

    // Pair oluştur ve yok et.
    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();
}
