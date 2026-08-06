# Tahmin Oyunu Programlama

Hadi birlikte uygulamalı bir proje yaparak Rust'a giriş yapalım! Bu bölüm, bazı
yaygın Rust kavramlarını gerçek bir programda nasıl kullanacağını göstererek
tanıtır. `let`, `match`, metodlar, ilişkili fonksiyonlar, harici crate'ler ve
daha fazlasını öğreneceksin! Sonraki bölümlerde bu fikirleri daha ayrıntılı
inceleyeceğiz. Bu bölümde yalnızca temelleri uygulayacaksın.

Klasik bir başlangıç programlama problemi olan tahmin oyununu uygulayacağız.
İşte nasıl çalıştığı: Program 1 ile 100 arasında rastgele bir tam sayı
üretecek. Ardından oyuncudan bir tahmin girmesini isteyecek. Bir tahmin
girildikten sonra program tahminin çok düşük mü yoksa çok yüksek mi olduğunu
belirtecek. Tahmin doğruysa oyun bir kutlama mesajı yazdırıp sona erecek.

## Yeni Proje Oluşturma

Yeni bir proje oluşturmak için Bölüm 1'de oluşturduğun _projects_ dizinine git
ve Cargo ile şöyle yeni bir proje oluştur:

```console
$ cargo new guessing_game
$ cd guessing_game
```

İlk komut olan `cargo new`, ilk bağımsız değişken olarak projenin adını
(`guessing_game`) alır. İkinci komut yeni projenin dizinine geçer.

Oluşturulan _Cargo.toml_ dosyasına bak:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial
rm -rf no-listing-01-cargo-new
cargo new no-listing-01-cargo-new --name guessing_game
cd no-listing-01-cargo-new
cargo run > output.txt 2>&1
cd ../../..
-->

<span class="filename">Dosya adı: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/Cargo.toml}}
```

Bölüm 1'de gördüğün gibi, `cargo new` senin için bir "Merhaba, dünya!" programı
üretir. _src/main.rs_ dosyasına göz at:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/src/main.rs}}
```

Şimdi bu "Merhaba, dünya!" programını derleyip aynı adımda `cargo run` komutunu
kullanarak çalıştıralım:

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/output.txt}}
```

`run` komutu, bu oyunda da yapacağımız gibi bir proje üzerinde hızlıca
iterasyon yapman, bir sonrakine geçmeden önce her iterasyonu hızlıca test
etmen gerektiğinde işine yarar.

_src/main.rs_ dosyasını yeniden aç. Tüm kodu bu dosyaya yazacaksın.

## Tahmini İşleme

Tahmin oyunu programının ilk kısmı kullanıcı girdisini isteyecek, bu girdiyi
işleyecek ve girdinin beklenen biçimde olup olmadığını kontrol edecek. Başlamak
için oyuncunun bir tahmin girmesine izin verelim. Liste 2-1'deki kodu
_src/main.rs_ dosyasına gir.

<Listing number="2-1" file-name="src/main.rs" caption="Kullanıcıdan bir tahmin alıp bunu yazdıran kod">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:all}}
```

</Listing>

Bu kod bir sürü bilgi içeriyor, o yüzden satır satır inceleyelim. Kullanıcı
girdisi almak ve sonucu çıktı olarak yazdırmak için `io` giriş/çıkış
kütüphanesini kapsamımıza almamız gerekiyor. `io` kütüphanesi `std` olarak
bilinen standart kütüphaneden gelir:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:io}}
```

Rust, varsayılan olarak standart kütüphanede tanımlı ve her programın kapsamına
getirdiği bir dizi öğeye sahiptir. Bu kümeye _prelude_ denir ve içindekilerin
tümünü [standart kütüphane belgelerinde][prelude] görebilirsin.

Kullanmak istediğin bir tür prelude'da değilse, o türü bir `use` ifadesiyle
açıkça kapsama almak zorundasın. `std::io` kütüphanesini kullanmak sana
kullanıcı girdisi kabul etme yeteneği de dahil olmak üzere birçok yararlı
özellik sağlar.

Bölüm 1'de gördüğün gibi, `main` fonksiyonu programın giriş noktasıdır:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:main}}
```

`fn` sözdizimi yeni bir fonksiyon tanımlar; parantezler (`()`) hiçbir
parametre olmadığını belirtir; süslü parantez (`{`) ise fonksiyonun gövdesini
başlatır.

Bölüm 1'de de öğrendiğin gibi, `println!` ekrana bir string yazdıran bir
makrodur:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print}}
```

Bu kod oyunun ne olduğunu belirten bir bilgi yazdırır ve kullanıcıdan girdi
ister.

### Değerleri Değişkenlerle Saklama

Sonra, kullanıcı girdisini saklamak için bir _değişken_ oluşturacağız, şöyle:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:string}}
```

Artık program ilginçleşiyor! Bu küçük satırda çok şey var. Değişkeni
oluşturmak için `let` ifadesini kullanıyoruz. İşte başka bir örnek:

```rust,ignore
let apples = 5;
```

Bu satır `apples` adında yeni bir değişken oluşturur ve onu `5` değerine
bağlar. Rust'ta değişkenler varsayılan olarak değiştirilemezdir; yani bir
değişkene bir değer verdiğimizde bu değer değişmeyecektir. Bu kavramı Bölüm
3'teki [“Değişkenler ve Değiştirilebilirlik”][variables-and-mutability]<!--
ignore --> bölümünde ayrıntılı olarak tartışacağız. Bir değişkeni
değiştirilebilir yapmak için değişken adından önce `mut` ekleriz:

```rust,ignore
let apples = 5; // değiştirilemez
let mut bananas = 5; // değiştirilebilir
```

> Not: `//` sözdizimi satır sonuna kadar devam eden bir yorum başlatır. Rust
> yorumlardaki her şeyi yok sayar. Yorumları [Bölüm 3][comments]<!-- ignore -->'te
> daha ayrıntılı tartışacağız.

Tahmin oyunu programına dönersek, artık `let mut guess`'in `guess` adında
değiştirilebilir bir değişken tanıtacağını biliyorsun. Eşittir işareti (`=`)
Rust'a şu anda değişkene bir şey bağlamak istediğimizi söyler. Eşittir
işaretinin sağında `guess`'in bağlandığı değer vardır; bu değer `String::new`
çağrısının sonucudur; `String::new`, yeni bir `String` örneği döndüren bir
fonksiyondur. [`String`][string]<!-- ignore -->, standart kütüphanenin
sağladığı, büyüyebilen, UTF-8 kodlu bir metin parçası olan bir string
türüdür.

`::new` satırındaki `::` sözdizimi, `new`'in `String` türünün ilişkili bir
fonksiyonu olduğunu belirtir. Bir _ilişkili fonksiyon_, bir tür üzerinde
uygulanan fonksiyondur; bu durumda `String` üzerinde. Bu `new` fonksiyonu yeni,
boş bir string oluşturur. Bir türdeki yeni bir değeri üreten fonksiyon için
yaygın bir isim olduğundan birçok türde bir `new` fonksiyonu bulacaksın.

Özetle, `let mut guess = String::new();` satırı şu anda yeni, boş bir `String`
örneğine bağlı değiştirilebilir bir değişken oluşturdu. Vay!

### Kullanıcı Girdisi Alma

Programın ilk satırında `use std::io;` ile standart kütüphaneden giriş/çıkış
işlevselliğini dahil ettiğimizi hatırla. Şimdi kullanıcı girdisini işlememizi
sağlayacak `io` modülünden `stdin` fonksiyonunu çağıracağız:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:read}}
```

Programın başında `use std::io;` ile `io` modülünü içe aktarmamış olsaydık, bu
fonksiyon çağrısını `std::io::stdin` olarak yazarak yine de kullanabilirdik.
`stdin` fonksiyonu, terminalinin standart girdisi için bir tanıtıcıyı (handle)
temsil eden bir tür olan [`std::io::Stdin`][iostdin]<!-- ignore --> örneği
döndürür.

Sonra, `.read_line(&mut guess)` satırı, standart girdi tanıtıcısı üzerindeki
[`read_line`][read_line]<!-- ignore --> metodunu çağırarak kullanıcıdan girdi
alır. Ayrıca `read_line`'a, kullanıcı girdisini hangi string'de saklayacağını
söylemek için bağımsız değişken olarak `&mut guess` değerini geçiriyoruz.
`read_line`'ın tam görevi, kullanıcının standart girdiye yazdığı her şeyi alıp
bir string'e eklemektir (içeriğinin üzerine yazmadan); bu yüzden o string'i
bağımsız değişken olarak geçiyoruz. String bağımsız değişkeninin
değiştirilebilir olması gerekir ki metod string'in içeriğini değiştirebilsin.

`&`, bu bağımsız değişkenin bir _referans_ olduğunu belirtir; bu da kodunun
birden çok bölümünün, veriyi belleğe birden çok kez kopyalamadan tek bir veri
parçasına erişmesini sağlar. Referanslar karmaşık bir özelliktir ve Rust'ın en
büyük avantajlarından biri referansları kullanmanın ne kadar güvenli ve kolay
olmasıdır. Bu programı bitirmek için o ayrıntıların çoğunu bilmene gerek yok.
Şimdilik bilmen gereken tek şey, değişkenler gibi referansların da varsayılan
olarak değiştirilemez olduğu. Bu yüzden onu değiştirilebilir yapmak için
`&guess` değil `&mut guess` yazman gerekir. (Bölüm 4 referansları daha
kapsamlı açıklayacak.)

<!-- Old headings. Do not remove or links may break. -->

<a id="handling-potential-failure-with-result"></a>

### `Result` ile Olası Hataları Yönetme

Hâlâ bu kod satırı üzerinde çalışıyoruz. Şimdi üçüncü bir metin satırını
tartışıyoruz; ancak bunun tek bir mantıksal kod satırının hâlâ bir parçası
olduğuna dikkat et. Sıradaki kısım şu metod:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:expect}}
```

Bu kodu şöyle de yazabilirdik:

```rust,ignore
io::stdin().read_line(&mut guess).expect("Satır okunamadı");
```

Ancak tek bir uzun satırın okunması zordur, bu yüzden onu bölmek en iyisidir.
Bir metodu `.method_name()` sözdizimiyle çağırdığında uzun satırları bölmek
için yeni satır ve diğer boşlukları eklemek genellikle akıllıcadır. Şimdi bu
satırın ne yaptığını tartışalım.

Daha önce de belirtildiği gibi, `read_line` kullanıcının girdiği her şeyi
geçtiğimiz string'e koyar, ama aynı zamanda bir `Result` değeri de döndürür.
[`Result`][result]<!-- ignore --> bir [_enum_][enums]<!-- ignore --> türüdür;
genellikle _enum_ olarak adlandırılır ve birden çok olası durumdan birinde
olabilen bir türdür. Her olası duruma _varyant_ deriz.

[Bölüm 6][enums]<!-- ignore --> enums'ları daha ayrıntılı olarak ele alacak. Bu
`Result` türlerinin amacı hata yönetimi bilgisini kodlamaktır.

`Result`'ın varyantları `Ok` ve `Err`'dür. `Ok` varyantı işlemin başarılı
olduğunu belirtir ve başarıyla üretilen değeri içerir. `Err` varyantı işlemin
başarısız olduğu anlamına gelir ve işlemin nasıl ya da neden başarısız olduğu
hakkında bilgi içerir.

Her türün değerleri gibi `Result` türünün değerlerinin de tanımlı metodları
vardır. Bir `Result` örneğinin çağırabileceğin bir [`expect`
metodu][expect]<!-- ignore --> vardır. Bu `Result` örneği bir `Err` değeriyse,
`expect` programın çökmesine neden olur ve `expect`'e bağımsız değişken olarak
geçtiğin mesajı görüntüler. `read_line` metodu bir `Err` döndürürse, bu
büyük olasılıkla temeldeki işletim sisteminden gelen bir hatanın sonucudur.
Bu `Result` örneği bir `Ok` değeriyse, `expect` `Ok`'un tuttuğu dönüş değerini
alır ve kullanman için yalnızca o değeri döndürür. Bu durumda bu değer,
kullanıcı girdisindeki bayt sayısıdır.

`expect`'i çağırmazsan program derlenir, ama bir uyarı alırsın:

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-02-without-expect/output.txt}}
```

Rust, `read_line`'dan döndürülen `Result` değerini kullanmadığını belirterek
programın olası bir hatayı ele almadığını belirtir.

Uyarıyı bastırmanın doğru yolu gerçekten hata yönetimi kodu yazmaktır, ama
bizim durumumuzda bir sorun oluştuğunda bu programı çökertmek istiyoruz, bu
yüzden `expect` kullanabiliriz. Hatalardan kurtulmayı [Bölüm 9][recover]<!--
ignore -->'da öğreneceksin.

### `println!` Yer Tutucularıyla Değer Yazdırma

Kapanış süslü parantezi dışında şu ana kadarki kodda tartışılacak yalnızca bir
satır daha kaldı:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print_guess}}
```

Bu satır artık kullanıcının girdisini içeren string'i yazdırır. `{}` süslü
parantezleri bir yer tutucudur: `{}`'yi bir değeri yerinde tutan küçük yengeç
kıskaçları gibi düşün. Bir değişkenin değerini yazdırırken değişken adı süslü
parantezlerin içine girebilir. Bir ifadenin değerlendirilmesinin sonucunu
yazdırırken biçimlendirme string'ine boş süslü parantezler koy, ardından
biçimlendirme string'ini, her boş süslü parantez yer tutucusuna aynı sırada
yazdırılacak virgülle ayrılmış ifadeler listesi izlesin. Bir değişkeni ve bir
ifadenin sonucunu tek bir `println!` çağrısında yazdırmak şöyle görünür:

```rust
let x = 5;
let y = 10;

println!("x = {x} ve y + 2 = {}", y + 2);
```

Bu kod `x = 5 ve y + 2 = 12` yazdırır.

### İlk Kısmı Test Etme

Tahmin oyununun ilk kısmını test edelim. `cargo run` kullanarak çalıştır:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-01/
cargo clean
cargo run
input 6 -->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.44s
     Running `target/debug/guessing_game`
Sayıyı tahmin et!
Lütfen tahminini gir.
6
Tahminin: 6
```

Bu noktada oyunun ilk kısmı tamamlandı: Klavyeden girdi alıyor ve sonra onu
yazdırıyoruz.

## Gizli Sayı Üretme

Sırada, kullanıcının tahmin etmeye çalışacağı gizli bir sayı üretmemiz
gerekiyor. Gizli sayı her seferinde farklı olmalı ki oyunu birden çok kez
oynamak eğlenceli olsun. Oyunun çok zor olmaması için 1 ile 100 arasında
rastgele bir sayı kullanacağız. Rust henüz standart kütüphanesine rastgele
sayı işlevselliğini dahil etmiyor. Ancak Rust ekibi, söz konusu işlevselliğe
sahip bir [`rand` crate'i][randcrate] sağlar.

<!-- Old headings. Do not remove or links may break. -->
<a id="using-a-crate-to-get-more-functionality"></a>

### Bir Crate ile İşlevselliği Artırma

Bir crate'in, Rust kaynak kodu dosyalarının bir koleksiyonu olduğunu hatırla.
Üzerinde çalıştığımız proje bir ikili (binary) crate'tir; yani çalıştırılabilir
bir dosyadır. `rand` crate'i ise bir kütüphane crate'idir; yani diğer
programlarda kullanılması amaçlanan ve tek başına çalıştırılamayan kodu içerir.

Cargo'nun harici crate'leri koordine etmesi, Cargo'nun gerçekten parladığı
yerdir. `rand` kullanan kod yazabilmemiz için `rand` crate'ini bir bağımlılık
olarak dahil etmek üzere _Cargo.toml_ dosyasını değiştirmemiz gerekiyor. O
dosyayı şimdi aç ve Cargo'nun senin için oluşturduğu `[dependencies]` bölüm
başlığının altına, alta şu satırı ekle. `rand`'ı tam olarak burada
belirttiğimiz gibi, bu sürüm numarasıyla belirttiğinden emin ol; aksi halde bu
öğreticideki kod örnekleri çalışmayabilir:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:

* ch01-01-installation.md
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
* ch14-03-cargo-workspaces.md
-->

<span class="filename">Dosya adı: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:8:}}
```

_Cargo.toml_ dosyasında bir başlığı izleyen her şey, başka bir bölüm
başlayana kadar devam eden o bölümün parçasıdır. `[dependencies]` içinde
Cargo'ya projenin hangi harici crate'lere bağımlı olduğunu ve bu crate'lerin
hangi sürümlerini gerektirdiğini söylersin. Bu durumda `rand` crate'ini anlamsal
sürüm belirleyici `0.10.1` ile belirtiyoruz. Cargo, sürüm numaraları yazmak
için bir standart olan [Anlamsal Sürümleme][semver]<!-- ignore -->'yi (bazen
_SemVer_ olarak da adlandırılır) anlar. `0.10.1` belirleyicisi aslında
`^0.10.1`'in kısaltmasıdır; bu, en az 0.10.1 ama 0.11.0'ın altında olan herhangi
bir sürüm anlamına gelir.

Cargo bu sürümlerin 0.10.1 ile uyumlu genel API'lere sahip olduğunu kabul eder
ve bu belirleme, bu bölümdeki kodla hâlâ derlenecek en son düzeltme (patch)
sürümünü almanı sağlar. 0.11.0 veya daha büyük herhangi bir sürümün, aşağıdaki
örneklerin kullandığıyla aynı API'ye sahip olacağı garanti edilmez.

Şimdi, kodu hiç değiştirmeden projeyi Liste 2-2'de gösterildiği gibi
derleyelim.

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
rm Cargo.lock
cargo clean
cargo build -->

<Listing number="2-2" caption="`rand` crate'ini bağımlılık olarak ekledikten sonra `cargo build` çalıştırmanın çıktısı">

```console
$ cargo build
    Updating crates.io index
     Locking 8 packages to latest Rust 1.96.0 compatible versions
  Downloaded rand_core v0.10.1
  Downloaded chacha20 v0.10.1
  Downloaded rand v0.10.1
  Downloaded 3 crates (162.9KiB) in 0.59s
   Compiling libc v0.2.186
   Compiling rand_core v0.10.1
   Compiling getrandom v0.4.3
   Compiling cfg-if v1.0.4
   Compiling chacha20 v0.10.1
   Compiling rand v0.10.1
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.03s
```

</Listing>

Farklı sürüm numaraları (ama SemVer sayesinde hepsi kodla uyumlu olacaktır!) ve
farklı satırlar (işletim sistemine bağlı olarak) görebilirsin ve satırların
sırası farklı olabilir.

Harici bir bağımlılık eklediğimizde, Cargo o bağımlılığın ihtiyaç duyduğu her
şeyin en son sürümlerini, [Crates.io][cratesio]'daki verilerin bir kopyası olan
_kayıt defterinden_ (registry) getirir. Crates.io, Rust ekosistemindeki
insanların açık kaynaklı Rust projelerini başkalarının kullanması için
yayınladıkları yerdir.

Kayıt defterini güncelledikten sonra Cargo `[dependencies]` bölümünü kontrol
eder ve henüz indirilmemiş listedeki crate'leri indirir. Bu durumda yalnızca
`rand`'ı bağımlılık olarak listelemiş olsak da Cargo, `rand`'ın çalışmak için
bağlı olduğu diğer crate'leri de kaptı. Crate'leri indirdikten sonra Rust
onları derler ve bağımlılıklar kullanılabilir durumdayken projeyi derler.

Hemen hiçbir değişiklik yapmadan `cargo build`'i yeniden çalıştırırsan,
`Finished` satırı dışında herhangi bir çıktı alamazsın. Cargo bağımlılıkları
zaten indirip derlediğini bilir ve _Cargo.toml_ dosyanda onlarla ilgili hiçbir
şeyi değiştirmedin. Cargo ayrıca kodunla ilgili hiçbir şeyi değiştirmediğini
bilir, bu yüzden onu da yeniden derlemez. Yapacak bir şey olmadığından basitçe
çıkar.

_src/main.rs_ dosyasını açarsan, önemsiz bir değişiklik yapıp kaydeder ve
yeniden derlersen yalnızca iki satır çıktı görürsün:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
touch src/main.rs
cargo build -->

```console
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
```

Bu satırlar Cargo'nun yalnızca _src/main.rs_ dosyasındaki küçük değişikliğinle
derlemeyi güncellediğini gösterir. Bağımlılıkların değişmedi, bu yüzden Cargo
onlar için zaten indirip derlediği şeyleri yeniden kullanabileceğini bilir.

<!-- Old headings. Do not remove or links may break. -->
<a id="ensuring-reproducible-builds-with-the-cargo-lock-file"></a>

#### Tekrarlanabilir Derlemeleri Sağlama

Cargo'nun, sen ya da başka biri kodunu her derlediğinde aynı yapıtı yeniden
derleyebilmeni sağlayan bir mekanizması vardır: Cargo, aksini belirtmediğin
sürece yalnızca belirttiğin bağımlılık sürümlerini kullanır. Örneğin, diyelim
ki gelecek hafta `rand` crate'inin 0.10.2 sürümü çıktı ve bu sürüm önemli bir
hata düzeltmesi içeriyor, ama aynı zamanda kodunu bozacak bir gerileme
(regression) da içeriyor. Bunu ele almak için Rust, `cargo build`'i ilk kez
çalıştırdığında _Cargo.lock_ dosyasını oluşturur; böylece şimdi bunu
_guessing_game_ dizininde buluruz.

Bir projeyi ilk kez derlediğinde Cargo, kriterlere uyan tüm bağımlılık
sürümlerini bulur ve sonra bunları _Cargo.lock_ dosyasına yazar. Gelecekte
projeni derlediğinde Cargo, _Cargo.lock_ dosyasının var olduğunu görür ve
sürümleri yeniden bulma işini yapmak yerine orada belirtilen sürümleri
kullanır. Bu, otomatik olarak tekrarlanabilir bir derlemeye sahip olmanı
sağlar. Başka bir deyişle, _Cargo.lock_ dosyası sayesinde projen açıkça
yükseltme yapana kadar 0.10.1'de kalır. _Cargo.lock_ dosyası tekrarlanabilir
derlemeler için önemli olduğundan, genellikle projenin geri kalan koduyla
birlikte kaynak kontrolüne (source control) eklenir.

#### Yeni Sürüm Almak için Bir Crate'i Güncelleme

Bir crate'i gerçekten güncellemek istediğinde Cargo, _Cargo.lock_ dosyasını yok
sayacak ve _Cargo.toml_ dosyandaki belirlemelerine uyan tüm en son sürümleri
bulacak `update` komutunu sağlar. Cargo sonra bu sürümleri _Cargo.lock_
dosyasına yazar. Aksi halde, Cargo varsayılan olarak yalnızca 0.10.1'den büyük
ve 0.11.0'dan küçük sürümleri arar. `rand` crate'i iki yeni sürüm olan 0.10.2 ve
0.999.0'ı yayınladıysa, `cargo update`'i çalıştırdığında şunları görürsün:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
cargo update
assuming there is a new version of rand; otherwise use another update
as a guide to creating the hypothetical output shown here -->

```console
$ cargo update
    Updating crates.io index
     Locking 1 package to latest Rust 1.96.0 compatible version
    Updating rand v0.10.1 -> v0.10.2 (available: v0.999.0)
```

Cargo 0.999.0 sürümünü yok sayar. Bu noktada _Cargo.lock_ dosyanda artık
kullandığın `rand` crate sürümünün 0.10.2 olduğunu belirten bir değişiklik de
fark edersin. `rand` sürümü 0.999.0'ı ya da 0.999._x_ dizisindeki herhangi bir
sürümü kullanmak için _Cargo.toml_ dosyasını bunun yerine şöyle görünecek
şekilde güncellemen gerekir (bunu gerçekten yapma çünkü aşağıdaki örnekler
`rand` 0.10 kullandığını varsayar):

```toml
[dependencies]
rand = "0.999.0"
```

Bir sonraki `cargo build`'i çalıştırdığında Cargo, kullanılabilir crate'lerin
kayıt defterini güncelleyecek ve `rand` gereksinimlerini belirttiğin yeni
sürüme göre yeniden değerlendirecek.

[Cargo][doccargo]<!-- ignore --> ve [ekosistemi][doccratesio]<!-- ignore -->
hakkında söylenecek çok daha fazla şey var; bunları Bölüm 14'te tartışacağız,
ama şimdilik bilmen gereken bu kadar. Cargo kütüphaneleri yeniden kullanmayı
çok kolaylaştırır; böylece Rustaceans'lar bir dizi paketten birleştirilen daha
küçük projeler yazabilirler.

### Rastgele Sayı Üretme

Tahmin edilecek bir sayı üretmek için `rand`'ı kullanmaya başlayalım. Sonraki
adım, _src/main.rs_ dosyasını Liste 2-3'te gösterildiği gibi güncellemek.

<Listing number="2-3" file-name="src/main.rs" caption="Rastgele sayı üretmek için kod ekleme">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:all}}
```

</Listing>

Önce `use rand::prelude::*;` satırını ekleriz. `prelude` modülü `rand`
crate'inin en çok kullanılan kısımlarını içerir ve `use` bu öğeleri
programımızın kapsamında kullanılabilir kılar.

Sonra ortaya iki satır ekliyoruz. İlk satırda, kullanacağımız belirli rastgele
sayı üretecini bize veren `rand::rng` fonksiyonunu çağırıyoruz: geçerli yürütme
iş parçacığına özel olan ve işletim sistemi tarafından tohumlanan (seed) bir
üreteç. Ardından rastgele sayı üreteci üzerindeki `random_range` metodunu
çağırıyoruz. Bu metod, `use rand::prelude::*;` ifadesiyle kapsama aldığımız
`rand::prelude` modülünün parçası olan `RngExt` trait'i tarafından tanımlanır.
`random_range` metodu bağımsız değişken olarak bir aralık (range) ifadesi alır
ve aralık içinde rastgele bir sayı üretir. Burada kullandığımız aralık ifadesi
türü `start..=end` biçimini alır ve alt ile üst sınırlar dahildir; bu yüzden 1
ile 100 arasında bir sayı istemek için `1..=100` belirtmemiz gerekir.

> Not: Bir crate'ten kapsama ne alacağını ve hangi metodları ve fonksiyonları
> çağıracağını sadece bilemezsin, bu yüzden her crate'in onu kullanmak için
> talimatlar içeren dokümantasyonu vardır. Cargo'nun bir başka güzel özelliği
> de `cargo doc --open` komutunu çalıştırmanın tüm bağımlılıkların sağladığı
> dokümantasyonu yerel olarak oluşturup tarayıcında açmasıdır. Örneğin `rand`
> crate'indeki diğer işlevlerle ilgileniyorsan `cargo doc --open` komutunu
> çalıştır ve sol taraftaki kenar çubuğunda `rand`'a tıkla.

İkinci yeni satır gizli sayıyı yazdırır. Program geliştirilirken onu test
edebilmek için bu kullanışlıdır, ama son sürümden sileceğiz. Program başlar
başlamaz cevabı yazdırıyorsa pek oyun sayılmaz!

Programı birkaç kez çalıştırmayı dene:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-03/
cargo run
4
cargo run
5
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
Sayıyı tahmin et!
Gizli sayı: 7
Lütfen tahminini gir.
4
Tahminin: 4

$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
Sayıyı tahmin et!
Gizli sayı: 83
Lütfen tahminini gir.
5
Tahminin: 5
```

Farklı rastgele sayılar almalısın ve hepsi 1 ile 100 arasında sayılar olmalı.
Uyarılar alırsan, bunları yok saymak güvenlidir. Hatalar alırsan, *Cargo.toml*
dosyanda `rand = "0.10.1"` olduğunu kontrol et; çünkü `rand`'ın gelecekteki
sürümlerinin farklı bir API'si olabilir, ama `0.10` dizisindeki herhangi bir
sürüm bu bölümdeki kodla çalışmalıdır.

## Tahmini Gizli Sayıyla Karşılaştırma

Artık kullanıcı girdisine ve rastgele bir sayıya sahibiz, ikisini
karşılaştırabiliriz. Bu adım Liste 2-4'te gösteriliyor. Bu kodun henüz
derlenmeyeceğine dikkat et; açıklayacağız.

<Listing number="2-4" file-name="src/main.rs" caption="İki sayıyı karşılaştırmanın olası dönüş değerlerini yönetme">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-04/src/main.rs:here}}
```

</Listing>

Önce standart kütüphaneden `std::cmp::Ordering` adında bir türü kapsama alan
başka bir `use` ifadesi ekleriz. `Ordering` türü başka bir enum'dur ve `Less`,
`Greater` ve `Equal` varyantlarına sahiptir. Bunlar iki değeri karşılaştırdığında
mümkün olan üç sonuçtur.

Sonra altta `Ordering` türünü kullanan beş yeni satır ekleriz. `cmp` metodu iki
değeri karşılaştırır ve karşılaştırılabilen her şey üzerinde çağrılabilir. Ne
ile karşılaştırmak istediğinin bir referansını alır: Burada `guess`'i
`secret_number` ile karşılaştırıyor. Ardından `use` ifadesiyle kapsama aldığımız
`Ordering` enum'unun bir varyantını döndürür. `guess` ve `secret_number`
değerleriyle `cmp` çağrısından hangi `Ordering` varyantının döndürüldüğüne göre
ne yapacağımıza karar vermek için bir [`match`][match]<!-- ignore --> ifadesi
kullanırız.

Bir `match` ifadesi _kollardan_ oluşur. Bir kol, eşleştirilecek bir _desenden_
ve `match`'e verilen değer o kolun desenine uyuyorsa çalıştırılacak koddan
oluşur. Rust `match`'e verilen değeri alır ve sırayla her kolun desenine bakar.
Desenler ve `match` yapısı güçlü Rust özellikleridir: Kodunun karşılaşabileceği
çeşitli durumları ifade etmene ve hepsini ele aldığından emin olmana izin
verirler. Bu özellikler sırasıyla Bölüm 6 ve Bölüm 19'da ayrıntılı olarak ele
alınacak.

Burada kullandığımız `match` ifadesiyle bir örnek üzerinden gidelim. Diyelim ki
kullanıcı 50 tahmin etti ve bu kez rastgele üretilen gizli sayı 38 olsun.

Kod 50'yi 38 ile karşılaştırdığında, `cmp` metodu 50, 38'den büyük olduğu için
`Ordering::Greater` döndürür. `match` ifadesi `Ordering::Greater` değerini alır
ve her kolun desenini kontrol etmeye başlar. İlk kolun desenine, `Ordering::Less`
desenine bakar ve `Ordering::Greater` değerinin `Ordering::Less` ile
eşleşmediğini görür; bu yüzden o koldaki kodu yok sayar ve sonraki kola geçer.
Sonraki kolun deseni `Ordering::Greater`'dır ve `Ordering::Greater` ile
_eşleşir_! O koldaki ilgili kod çalışır ve ekrana `Çok büyük!` yazdırır.
`match` ifadesi ilk başarılı eşleşmeden sonra sona erer, bu yüzden bu senaryoda
son kola bakmaz.

Ancak Liste 2-4'teki kod henüz derlenmeyecek. Deneyelim:

<!--
The error numbers in this output should be that of the code **WITHOUT** the
anchor or snip comments
-->

```console
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-04/output.txt}}
```

Hatanın özü, _eşleşmeyen türler_ (mismatched types) olduğunu belirtir. Rust
güçlü, statik bir tür sistemine sahiptir. Ancak aynı zamanda tür çıkarımına da
sahiptir. `let mut guess = String::new()` yazdığımızda Rust, `guess`'in bir
`String` olması gerektiğini çıkarabildi ve türü yazmamızı gerektirmedi. Diğer
yandan `secret_number` bir sayı türüdür. Rust'ın birkaç sayı türü 1 ile 100
arasında bir değere sahip olabilir: `i32`, 32 bitlik bir sayı; `u32`, işaretsiz
bir 32 bitlik sayı; `i64`, 64 bitlik bir sayı; ve diğerleri. Aksi
belirtilmedikçe Rust varsayılan olarak `i32` kullanır; bu da Rust'ın farklı bir
sayısal tür çıkarmasına neden olacak başka bir yerde tür bilgisi eklemediğin
sürece `secret_number`'ın türüdür. Hatanın nedeni, Rust'ın bir string ile bir
sayı türünü karşılaştıramamasıdır.

Sonuçta programın girdi olarak okuduğu `String`'i bir sayı türüne dönüştürmek
istiyoruz ki onu gizli sayıyla sayısal olarak karşılaştırabilelim. Bunu
`main` fonksiyon gövdesine şu satırı ekleyerek yaparız:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/src/main.rs:here}}
```

Satır şudur:

```rust,ignore
let guess: u32 = guess.trim().parse().expect("Lütfen bir sayı yaz!");
```

`guess` adında bir değişken oluşturuyoruz. Ama dur, programın zaten `guess`
adında bir değişkeni yok mu? Var, ama Rust kullanışlı bir şekilde önceki
`guess` değerini yenisiyle gölgelememize izin verir. _Gölgeleme_, örneğin
`guess_str` ve `guess` gibi iki benzersiz değişken oluşturmak yerine `guess`
değişken adını yeniden kullanmamızı sağlar. Bunu [Bölüm 3][shadowing]<!--
ignore -->'te daha ayrıntılı ele alacağız, ama şimdilik bu özelliğin genellikle
bir değeri bir türden başka bir türe dönüştürmek istediğinde kullanıldığını
bil.

Bu yeni değişkeni `guess.trim().parse()` ifadesine bağlarız. İfadedeki `guess`,
girdiyi bir string olarak içeren orijinal `guess` değişkenine atıfta bulunur.
Bir `String` örneği üzerindeki `trim` metodu, yalnızca sayısal veri içerebilen
`u32`'ye dönüştürmeden önce yapmamız gereken, başlangıçtaki ve sondaki tüm
boşlukları ortadan kaldıracaktır. Kullanıcı `read_line`'ı karşılamak ve
tahminini girmek için <kbd>enter</kbd>'a basmalıdır; bu, string'e bir yeni satır
karakteri ekler. Örneğin kullanıcı <kbd>5</kbd> yazıp <kbd>enter</kbd>'a
basarsa, `guess` şöyle görünür: `5\n`. `\n` "yeni satır"ı temsil eder. (Windows'ta
<kbd>enter</kbd>'a basmak bir satır başı ve bir yeni satırla sonuçlanır:
`\r\n`.) `trim` metodu `\n` veya `\r\n`'yi ortadan kaldırır ve geriye yalnızca
`5` kalır.

[string'lerdeki `parse` metodu][parse]<!-- ignore --> bir string'i başka bir
türe dönüştürür. Burada onu bir string'den bir sayıya dönüştürmek için
kullanıyoruz. İstediğimiz tam sayı türünü Rust'a söylemek için `let guess: u32`
kullanmamız gerekiyor. `guess`'ten sonraki iki nokta (`:`) Rust'a değişkenin
türünü belirteceğimizi söyler. Rust'ın birkaç yerleşik sayı türü vardır;
burada görülen `u32` işaretsiz, 32 bitlik bir tam sayıdır. Küçük pozitif bir
sayı için iyi bir varsayılan seçimdir. Diğer sayı türlerini [Bölüm
3][integers]<!-- ignore -->'te öğreneceksin.

Ayrıca bu örnek programdaki `u32` belirtimi ve `secret_number` ile karşılaştırma,
Rust'ın `secret_number`'ın da bir `u32` olması gerektiğini çıkaracağı anlamına
gelir. Yani artık karşılaştırma aynı türden iki değer arasında olacak!

`parse` metodu yalnızca mantıksal olarak sayılara dönüştürülebilen karakterler
üzerinde çalışır ve bu yüzden kolayca hatalara neden olabilir. Örneğin, string
`A👍%` içerseydi, bunu bir sayıya dönüştürmenin hiçbir yolu olmazdı. Başarısız
olabileceği için `parse` metodu, `read_line` metodunun yaptığı gibi (daha önce
["`Result` ile Olası Hataları Yönetme"](#handling-potential-failure-with-result)<!--
ignore --> bölümünde tartışıldı) bir `Result` türü döndürür. Bu `Result`'ı,
`expect` metodunu yine kullanarak aynı şekilde ele alacağız. `parse` string'den
bir sayı oluşturamadığı için bir `Err` `Result` varyantı döndürürse, `expect`
çağrısı oyunu çökertir ve verdiğimiz mesajı yazdırır. `parse` string'i başarıyla
bir sayıya dönüştürebilirse `Result`'un `Ok` varyantını döndürür ve `expect`
bize `Ok` değerinden istediğimiz sayıyı döndürür.

Şimdi programı çalıştıralım:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/
touch src/main.rs
cargo run
  76
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s
     Running `target/debug/guessing_game`
Sayıyı tahmin et!
Gizli sayı: 58
Lütfen tahminini gir.
  76
Tahminin: 76
Çok büyük!
```

Güzel! Tahminin önüne boşluklar eklenmiş olsa bile program kullanıcının 76
tahmin ettiğini buldu. Farklı girdi türleriyle farklı davranışları doğrulamak
için programı birkaç kez çalıştır: Sayıyı doğru tahmin et, çok yüksek bir sayı
tahmin et ve çok düşük bir sayı tahmin et.

Oyunun çoğu artık çalışıyor, ama kullanıcı yalnızca bir tahmin yapabilir. Bunu
bir döngü ekleyerek değiştirelim!

## Döngüyle Birden Çok Tahmine İzin Verme

`loop` anahtar kelimesi sonsuz bir döngü oluşturur. Kullanıcılara sayıyı
tahmin etmeleri için daha fazla şans vermek üzere bir döngü ekleyeceğiz:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-04-looping/src/main.rs:here}}
```

Gördüğün gibi, tahmin girdisi isteminden itibaren her şeyi bir döngüye taşıdık.
Döngünün içindeki satırları her birini dört boşluk daha girintilediğinden emin
ol ve programı yeniden çalıştır. Program artık sonsuza dek başka bir tahmin
isteyecek; bu aslında yeni bir soruna yol açar. Kullanıcı çıkamıyor gibi
görünüyor!

Kullanıcı, <kbd>ctrl</kbd>-<kbd>C</kbd> klavye kısayolunu kullanarak her zaman
programı bölebilir. Ama bu doymak bilmez canavardan kaçmanın başka bir yolu
daha var; [“Tahmini Gizli Sayıyla Karşılaştırma”](#comparing-the-guess-to-the-secret-number)<!--
ignore --> bölümündeki `parse` tartışmasında değinildiği gibi: Kullanıcı sayı
olmayan bir cevap girerse program çöker. Buradan yararlanarak kullanıcının
çıkmasına izin verebiliriz, şurada gösterildiği gibi:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-04-looping/
touch src/main.rs
cargo run
(too small guess)
(too big guess)
(correct guess)
quit
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running `target/debug/guessing_game`
Sayıyı tahmin et!
Gizli sayı: 59
Lütfen tahminini gir.
45
Tahminin: 45
Çok küçük!
Lütfen tahminini gir.
60
Tahminin: 60
Çok büyük!
Lütfen tahminini gir.
59
Tahminin: 59
Kazandın!
Lütfen tahminini gir.
quit

thread 'main' (6694925) panicked at src/main.rs:28:47:
Lütfen bir sayı yaz!: ParseIntError { kind: InvalidDigit }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

`quit` yazmak oyundan çıkar, ama fark edeceğin gibi sayı olmayan başka herhangi
bir girdi girmek de öyle. Bu en hafif tabirle ideal değil; oyunun doğru sayı
tahmin edildiğinde de durmasını istiyoruz.

### Doğru Tahminden Sonra Çıkma

Oyunu, kullanıcı kazandığında çıkacak şekilde programlayalım; bir `break`
ifadesi ekleyerek:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-05-quitting/src/main.rs:here}}
```

`Kazandın!`'dan sonra `break` satırını eklemek, kullanıcı gizli sayıyı doğru
tahmin ettiğinde programın döngüden çıkmasını sağlar. Döngüden çıkmak aynı
zamanda programdan çıkmak anlamına gelir; çünkü döngü `main`'in son kısmıdır.

### Geçersiz Girdiyi Yönetme

Oyunun davranışını daha da iyileştirmek için, kullanıcı sayı olmayan bir girdi
girdiğinde programı çökertmek yerine oyunun sayı olmayanı yok saymasını
sağlayalım ki kullanıcı tahmin etmeye devam edebilsin. Bunu `guess`'in bir
`String`'den bir `u32`'ye dönüştürüldüğü satırı Liste 2-5'te gösterildiği gibi
değiştirerek yapabiliriz.

<Listing number="2-5" file-name="src/main.rs" caption="Sayı olmayan bir tahmini yok sayıp programı çökertmek yerine başka bir tahmin isteme">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-05/src/main.rs:here}}
```

</Listing>

Bir hatada çökmekten hatayı yönetmeye geçmek için `expect` çağrısından bir
`match` ifadesine geçiyoruz. `parse`'in bir `Result` türü döndürdüğünü ve
`Result`'ın `Ok` ve `Err` varyantlarına sahip bir enum olduğunu hatırla. Burada
`cmp` metodunun `Ordering` sonucunda yaptığımız gibi bir `match` ifadesi
kullanıyoruz.

`parse` string'i başarıyla bir sayıya dönüştürebilirse, sonucu sayıyı içeren bir
`Ok` değeri döndürür. Bu `Ok` değeri ilk kolun deseniyle eşleşir ve `match`
ifadesi, `parse`'in üretip `Ok` değerinin içine koyduğu `num` değerini yalnızca
döndürür. Bu sayı, oluşturduğumuz yeni `guess` değişkeninde istediğimiz yerde
olacak.

`parse` string'i bir sayıya dönüştüremezse, hata hakkında daha fazla bilgi
içeren bir `Err` değeri döndürür. `Err` değeri ilk `match` kolundaki `Ok(num)`
deseniyle eşleşmez, ama ikinci koldaki `Err(_)` deseniyle eşleşir. Alt çizgi
(`_`) her şeyi yakalayan bir değerdir; bu örnekte, içlerinde ne bilgi olursa
olsun tüm `Err` değerleriyle eşleşmek istediğimizi söylüyoruz. Böylece program
ikinci kolun kodunu, `continue`'ı çalıştırır; bu da programa `loop`'un sonraki
iterasyonuna gitmesini ve başka bir tahmin istemesini söyler. Yani `parse`'in
karşılaşabileceği tüm hataları etkili bir şekilde program yok sayar!

Artık programdaki her şey beklendiği gibi çalışmalı. Deneyelim:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-05/
cargo run
(too small guess)
(too big guess)
foo
(correct guess)
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/guessing_game`
Sayıyı tahmin et!
Gizli sayı: 61
Lütfen tahminini gir.
10
Tahminin: 10
Çok küçük!
Lütfen tahminini gir.
99
Tahminin: 99
Çok büyük!
Lütfen tahminini gir.
foo
Lütfen tahminini gir.
61
Tahminin: 61
Kazandın!
```

Harika! Küçük son bir rötuşla tahmin oyununu bitireceğiz. Programın hâlâ gizli
sayıyı yazdırdığını hatırla. Bu test için işe yaradı, ama oyunu bozuyor. Gizli
sayıyı çıktı olarak veren `println!`'ı silelim. Liste 2-6 son kodu gösteriyor.

<Listing number="2-6" file-name="src/main.rs" caption="Tahmin oyununun tamamlanmış kodu">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-06/src/main.rs}}
```

</Listing>

Bu noktada tahmin oyununu başarıyla oluşturdun. Tebrikler!

## Özet

Bu proje sana birçok yeni Rust kavramını tanıtmak için uygulamalı bir yoldu:
`let`, `match`, fonksiyonlar, harici crate'lerin kullanımı ve daha fazlası.
Sonraki birkaç bölümde bu kavramları daha ayrıntılı öğreneceksin. Bölüm 3,
çoğu programlama dilinin sahip olduğu değişkenler, veri türleri ve fonksiyonlar
gibi kavramları ele alır ve bunların Rust'ta nasıl kullanılacağını gösterir.
Bölüm 4, Rust'ı diğer dillerden farklı kılan bir özellik olan sahipliği
(ownership) inceler. Bölüm 5 struct'ları ve metod sözdizimini tartışır ve
Bölüm 6 enum'ların nasıl çalıştığını açıklar.

[prelude]: ../std/prelude/index.html
[variables-and-mutability]: ch03-01-variables-and-mutability.html#variables-and-mutability
[comments]: ch03-04-comments.html
[string]: ../std/string/struct.String.html
[iostdin]: ../std/io/struct.Stdin.html
[read_line]: ../std/io/struct.Stdin.html#method.read_line
[result]: ../std/result/enum.Result.html
[enums]: ch06-00-enums.html
[expect]: ../std/result/enum.Result.html#method.expect
[recover]: ch09-02-recoverable-errors-with-result.html
[randcrate]: https://crates.io/crates/rand
[semver]: http://semver.org
[cratesio]: https://crates.io/
[doccargo]: https://doc.rust-lang.org/cargo/
[doccratesio]: https://doc.rust-lang.org/cargo/reference/publishing.html
[match]: ch06-02-match.html
[shadowing]: ch03-01-variables-and-mutability.html#shadowing
[parse]: ../std/primitive.str.html#method.parse
[integers]: ch03-02-data-types.html#integer-types
