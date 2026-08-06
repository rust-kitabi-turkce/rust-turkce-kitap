## Merhaba, Cargo!

Cargo, Rust'ın derleme sistemi ve paket yöneticisidir. Çoğu Rustacean bu aracı
Rust projelerini yönetmek için kullanır çünkü Cargo sizin için birçok görevi
üstlenir: kodunuzu derlemek, kodunuzun bağımlı olduğu kütüphaneleri indirmek ve
bu kütüphaneleri derlemek gibi. (Kodunuzun ihtiyaç duyduğu kütüphanelere
_bağımlılıklar_ diyoruz.)

Şimdiye kadar yazdığımız gibi en basit Rust programlarının hiçbir bağımlılığı
yoktur. "Merhaba, dünya!" projesini Cargo ile oluşturup yarattıysak, Cargo'nun
yalnızca kodunuzu derleme işini yapan kısmını kullanabilirdi. Daha karmaşık Rust
programları yazdıkça bağımlılıklar eklersiniz ve bir projeye Cargo ile
başlarsanız, bağımlılık eklemek çok daha kolay olur.

Rust projelerinin büyük çoğunluğu Cargo kullandığı için, bu kitabın geri kalanı
sizin de Cargo kullandığınızı varsayar. [Kurulum][installation]<!-- ignore -->
bölümünde ele alınan resmi kurulumları kullandıysanız, Cargo Rust ile birlikte
kurulmuştur. Rust'ı başka bir yoldan kurduysanız, terminalinize şunu girerek
Cargo'nun kurulu olup olmadığını kontrol edin:

```console
$ cargo --version
```

Bir sürüm numarası görüyorsanız, Cargo kuruludur! `command not found` gibi bir
hata görüyorsanız, Cargo'yu ayrıca nasıl kuracağınızı belirlemek için kurulum
yönteminizin belgelerine bakın.

### Cargo ile Proje Oluşturma

Cargo'yu kullanarak yeni bir proje oluşturalım ve bunun orijinal "Merhaba,
dünya!" projemizden nasıl farklı olduğuna bakalım. _projects_ dizininize (ya da
kodunuzu saklamaya karar verdiğiniz her yere) geri dönün. Ardından, herhangi bir
işletim sisteminde şunu çalıştırın:

```console
$ cargo new hello_cargo
$ cd hello_cargo
```

İlk komut, _hello_cargo_ adında yeni bir dizin ve proje oluşturur. Projemize
_hello_cargo_ adını verdik ve Cargo, aynı adlı bir dizinde dosyalarını
oluşturur.

_hello_cargo_ dizinine gidin ve dosyaları listeleyin. Cargo'nun bizim için iki
dosya ve bir dizin oluşturduğunu görürsünüz: bir _Cargo.toml_ dosyası ve içinde
bir _main.rs_ dosyası bulunan bir _src_ dizini.

Ayrıca bir _.gitignore_ dosyasıyla birlikte yeni bir Git deposu da
başlatmıştır. `cargo new` komutunu mevcut bir Git deposunun içinde çalıştırırsanız
Git dosyaları oluşturulmaz; `cargo new --vcs=git` komutunu kullanarak bu
davranışı geçersiz kılabilirsiniz.

> Not: Git, yaygın bir sürüm kontrol sistemidir. `cargo new` komutunu `--vcs`
> bayrağıyla farklı bir sürüm kontrol sistemi kullanacak şekilde ya da hiçbir
> sürüm kontrol sistemi kullanmayacak şekilde değiştirebilirsiniz. Mevcut
> seçenekleri görmek için `cargo new --help` komutunu çalıştırın.

_Cargo.toml_ dosyasını seçtiğiniz metin editöründe açın. Liste 1-2'deki koda
benzer görünmelidir.

<Listing number="1-2" file-name="Cargo.toml" caption="`cargo new` tarafından oluşturulan *Cargo.toml* dosyasının içeriği">

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2024"

[dependencies]
```

</Listing>

Bu dosya, Cargo'nun yapılandırma biçimi olan [_TOML_][toml]<!-- ignore -->
(_Tom'un Açık, Minimal Dili_) biçimindedir.

İlk satır olan `[package]`, aşağıdaki ifadelerin bir paketi yapılandırdığını
gösteren bir bölüm başlığıdır. Bu dosyaya daha fazla bilgi ekledikçe, başka
bölümler de ekleyeceğiz.

Sonraki üç satır, Cargo'nun programınızı derlemek için ihtiyaç duyduğu
yapılandırma bilgisini ayarlar: kullanılacak ad, sürüm ve Rust edition'ı.
`edition` anahtarı hakkında [Ek E'de][appendix-e]<!-- ignore --> konuşacağız.

Son satır olan `[dependencies]`, projenizin bağımlılıklarından herhangi birini
listeleyebileceğiniz bir bölümün başlangıcıdır. Rust'ta kod paketlerine _crate_
denir. Bu proje için başka bir crate'e ihtiyacımız olmayacak, ancak 2.
Bölümdeki ilk projede ihtiyacımız olacak; bu yüzden bu bağımlılıklar bölümünü o
zaman kullanacağız.

Şimdi _src/main.rs_ dosyasını açın ve bir göz atın:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
fn main() {
    println!("Merhaba, dünya!");
}
```

Cargo, sizin için Liste 1-1'de yazdığımızla aynı olan bir "Merhaba, dünya!"
programı üretti! Şimdiye kadar projemiz ile Cargo'nun oluşturduğu arasındaki
farklar, Cargo'nun kodu _src_ dizinine yerleştirmesi ve en üst dizinde bir
_Cargo.toml_ yapılandırma dosyasına sahip olmamızdır.

Cargo, kaynak dosyalarınızın _src_ dizininin içinde yaşamasını bekler. En üst
düzey proje dizini, README dosyaları, lisans bilgileri, yapılandırma dosyaları
ve kodunuzla ilgisi olmayan diğer her şey içindir. Cargo kullanmak
projelerinizi düzenlemenize yardımcı olur. Her şey için bir yer vardır ve her
şey yerindedir.

Daha önce "Merhaba, dünya!" projesinde yaptığımız gibi Cargo'yu kullanmayan bir
proje başlattıysanız, onu Cargo kullanan bir projeye dönüştürebilirsiniz. Proje
kodunu _src_ dizinine taşıyın ve uygun bir _Cargo.toml_ dosyası oluşturun. O
_Cargo.toml_ dosyasını elde etmenin kolay bir yolu, onu sizin için otomatik
olarak oluşturacak olan `cargo init` komutunu çalıştırmaktır.

### Bir Cargo Projesini Derleme ve Çalıştırma

Şimdi "Merhaba, dünya!" programını Cargo ile derleyip çalıştırdığımızda neyin
farklı olduğuna bakalım! _hello_cargo_ dizininizden aşağıdaki komutu girerek
projenizi derleyin:

```console
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```

Bu komut, geçerli dizininizde değil de _target/debug/hello_cargo_ içinde
(Windows'ta _target\debug\hello_cargo.exe_) bir çalıştırılabilir dosya
oluşturur. Varsayılan derlemenin bir hata ayıklama (debug) derlemesi olması
nedeniyle, Cargo ikili dosyayı _debug_ adlı bir dizine koyar. Çalıştırılabilir
dosyayı şu komutla çalıştırabilirsiniz:

```console
$ ./target/debug/hello_cargo # veya Windows'ta .\target\debug\hello_cargo.exe
Merhaba, dünya!
```

Her şey yolunda giderse, `Merhaba, dünya!` metni terminale yazdırılmalıdır.
`cargo build` komutunu ilk kez çalıştırmak, Cargo'nun en üst düzeyde yeni bir
dosya oluşturmasına da neden olur: _Cargo.lock_. Bu dosya, projenizdeki
bağımlılıkların tam sürümlerini izler. Bu projenin bağımlılığı yoktur, bu
yüzden dosya biraz boştur. Bu dosyayı elle değiştirmeniz hiç gerekmez; Cargo,
içeriğini sizin için yönetir.

Az önce `cargo build` ile bir proje derledik ve `./target/debug/hello_cargo` ile
çalıştırdık, ancak kodu derlemek ve ardından oluşan çalıştırılabilir dosyayı
çalıştırmak için hepsini tek komutta yapabileceğimiz `cargo run` komutunu da
kullanabiliriz:

```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Merhaba, dünya!
```

`cargo run` kullanmak, `cargo build` komutunu çalıştırmayı ve ardından ikili
dosyanın tam yolunu kullanmayı hatırlamak zorunda olmaktan daha uygundur, bu
yüzden çoğu geliştirici `cargo run` kullanır.

Bu sefer Cargo'nun `hello_cargo`'yu derlediğine dair bir çıktı görmediğimizi
fark edin. Cargo, dosyaların değişmediğini anladı, bu yüzden yeniden derlemedi,
sadece ikili dosyayı çalıştırdı. Kaynak kodunuzu değiştirseydiniz, Cargo
projeyi çalıştırmadan önce yeniden derlerdi ve bu çıktıyı görürdünüz:

```console
$ cargo run
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs
     Running `target/debug/hello_cargo`
Merhaba, dünya!
```

Cargo ayrıca `cargo check` adında bir komut da sağlar. Bu komut, kodunuzun
derlendiğinden emin olmak için hızlıca kontrol eder ancak bir çalıştırılabilir
dosya üretmez:

```console
$ cargo check
   Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

Neden çalıştırılabilir bir dosya istemiyorsunuz? Çoğu zaman `cargo check`,
çalıştırılabilir bir dosya üretme adımını atladığı için `cargo build`'dan çok
daha hızlıdır. Kod yazarken çalışmanızı sürekli kontrol ediyorsanız, `cargo
check` kullanmak, projenizin hâlâ derlenip derlenmediğini size bildirme işlemini
hızlandırır! Bu nedenle, birçok Rustacean, programının derlendiğinden emin olmak
için programını yazarken periyodik olarak `cargo check` çalıştırır. Ardından,
çalıştırılabilir dosyayı kullanmaya hazır olduklarında `cargo build` çalıştırır.

Şimdiye kadar Cargo hakkında öğrendiklerimizi özetleyelim:

- `cargo new` kullanarak bir proje oluşturabiliriz.
- `cargo build` kullanarak bir projeyi derleyebiliriz.
- `cargo run` kullanarak bir projeyi tek adımda derleyip çalıştırabiliriz.
- `cargo check` kullanarak hataları kontrol etmek için ikili dosya üretmeden bir
  proje derleyebiliriz.
- Derlemenin sonucunu kodumuzla aynı dizinde saklamak yerine Cargo bunu
  _target/debug_ dizininde saklar.

Cargo kullanmanın ek bir avantajı da komutların hangi işletim sistemi üzerinde
çalışıyor olursanız olun aynı olmasıdır. Bu yüzden, bundan sonra Linux ve macOS
ile Windows için ayrı talimatlar vermeyeceğiz.

### Yayın (Release) için Derleme

Projeniz nihayet yayına hazır olduğunda, optimizasyonlarla derlemek için
`cargo build --release` komutunu kullanabilirsiniz. Bu komut, _target/debug_
yerine _target/release_ içinde bir çalıştırılabilir dosya oluşturur.
Optimizasyonlar Rust kodunuzun daha hızlı çalışmasını sağlar, ancak bunları
açmak programınızın derlenmesinin aldığı süreyi uzatır. Bu nedenle iki farklı
profil vardır: biri, hızlı ve sık yeniden derlemek istediğiniz geliştirme için;
diğeri, kullanıcıya vereceğiniz, yeniden derlenmesi gerekmeyen ve mümkün
olduğunca hızlı çalışacak nihai programı oluşturmak için. Kodunuzun çalışma
süresini kıyaslıyorsanız, `cargo build --release` komutunu çalıştırdığınızdan
emin olun ve _target/release_ içindeki çalıştırılabilir dosyayla kıyaslama
yapın.

<!-- Old headings. Do not remove or links may break. -->
<a id="cargo-as-convention"></a>

### Cargo'nun Kurallarından Yararlanma

Basit projelerde Cargo, `rustc` kullanmaya kıyasla fazla bir değer katmaz,
ancak programlarınız daha karmaşık hale geldikçe değerini kanıtlayacaktır.
Programlar birden fazla dosyaya büyüdüğünde ya da bir bağımlılığa ihtiyaç
duyduğunda, derlemeyi koordine etmek için Cargo'ya bırakmak çok daha kolaydır.

`hello_cargo` projesi basit olsa da, artık Rust kariyerinizin geri kalanında
kullanacağınız gerçek araçların çoğunu kullanır. Aslında, mevcut herhangi bir
proje üzerinde çalışmak için, kodu Git ile çekmek, o projenin dizinine geçmek ve
derlemek için şu komutları kullanabilirsiniz:

```console
$ git clone example.org/someproject
$ cd someproject
$ cargo build
```

Cargo hakkında daha fazla bilgi için [belgelerine][cargo] göz atın.

## Özet

Rust yolculuğunuza şimdiden harika bir başlangıç yaptınız! Bu bölümde şunları
öğrendiniz:

- `rustup` kullanarak Rust'ın en son kararlı sürümünü kurmak.
- Daha yeni bir Rust sürümüne güncellemek.
- Yerel olarak kurulu belgeleri açmak.
- `rustc` kullanarak doğrudan bir "Merhaba, dünya!" programı yazmak ve
  çalıştırmak.
- Cargo'nun kurallarını kullanarak yeni bir proje oluşturmak ve çalıştırmak.

Bu, Rust kodu okumaya ve yazmaya alışmak için daha kapsamlı bir program
geliştirmek için harika bir zamandır. Bu yüzden, 2. Bölümde bir sayı tahmin etme
oyunu programı oluşturacağız. Önce yaygın programlama kavramlarının Rust'ta nasıl
çalıştığını öğrenerek başlamayı tercih ederseniz, 3. Bölüme bakın ve sonra 2.
Bölüme dönün.

[installation]: ch01-01-installation.html#kurulum
[toml]: https://toml.io
[appendix-e]: appendix-05-editions.html
[cargo]: https://doc.rust-lang.org/cargo/
