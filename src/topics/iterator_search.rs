pub fn execute_iterator_search() {
    // İki vektör oluşturuluyor
    let first_vec = vec![1, 2, 3];
    let second_vec = vec![4, 5, 6];

    // first_vec üzerinde借用 (borrow) yoluyla bir iterator alınıyor; vektör kullanılabilir kalır
    let mut first_vec_iter = first_vec.iter();
    // second_vec sahipliği (ownership) iterator'a devrediliyor; artık vektöre erişilemez
    let mut second_into_iter = second_vec.into_iter();

    // iter() kullandığı için kapanış (closure) parametresi &&x biçimindedir (referansın referansı)
    println!(
        "Find 2 in first_vec: {:?}",
        first_vec_iter.find(|&&x| x == 2) // 2 bulunur: Some(2) yazdırılır
    );

    // into_iter() kullandığı için closure parametresi &x biçimindedir (değerin referansı)
    println!(
        "Find 2 in second_vec: {:?}",
        second_into_iter.find(|&x| x == 2) // 2 yok, o yüzden None yazdırılır
    );

    // Dizilerde de aynı mantık geçerli
    let my_first_array = [1, 2, 3];
    let my_second_array = [4, 5, 6];

    // Dizide iter() ile arama: closure &&x alır
    println!(
        "Find 2 in my_first_array: {:?}",
        my_first_array.iter().find(|&&x| x == 2) // Some(2) bulunur
    );

    // Dizide into_iter() ile arama: closure &x alır
    println!(
        "Find 2 in my_second_array: {:?}",
        my_second_array.into_iter().find(|&x| x == 2) // None döner
    );
}

pub fn execute_iterator_position() {
    // Örnek vektör oluşturuluyor
    let sample_vec = vec![1, 9, 3, 3, 13, 2];

    // position(): koşulu sağlayan ilk elemanın indeksini döndürür
    // İlk çift sayı olan 2, 5. indekstedir
    let index_of_first_even_number = sample_vec.iter().position(|&x| x % 2 == 0);

    // Sonuç Some(5) olmalı; assert ile doğrulanıyor
    assert_eq!(index_of_first_even_number, Some(5));

    // into_iter() ile sahiplik devredilir; negatif sayı olmadığı için sonuç None olur
    let index_of_rist_negative_number = sample_vec.into_iter().position(|x| x < 0);
    // Vektörde negatif sayı yoksa None döner
    assert_eq!(index_of_rist_negative_number, None);
}
