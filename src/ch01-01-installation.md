## Kurulum

İlk adım Rust'ı kurmak. Rust'ı, Rust sürümlerini ve ilgili araçları yöneten bir
komut satırı aracı olan `rustup` aracılığıyla indireceğiz. İndirme için internet
bağlantısına ihtiyacınız olacak.

> Not: Herhangi bir nedenle `rustup` kullanmayı tercih etmezseniz, daha fazla
> seçenek için [Diğer Rust Kurulum Yöntemleri sayfasına][otherinstall] bakın.

Aşağıdaki adımlar, Rust derleyicisinin en son kararlı sürümünü kurar. Rust'ın
kararlılık garantileri, kitapta derlenen tüm örneklerin daha yeni Rust
sürümleriyle derlenmeye devam edeceğini güvence altına alır. Rust hata
mesajlarını ve uyarılarını sık sık iyileştirdiği için çıktı sürümler arasında
biraz farklılık gösterebilir. Başka bir deyişle, bu adımları kullanarak
kurduğunuz herhangi bir daha yeni, kararlı Rust sürümü bu kitabın içeriğiyle
beklendiği gibi çalışmalıdır.

> ### Komut Satırı Gösterimi
>
> Bu bölümde ve kitap boyunca terminalde kullanılan bazı komutları
> göstereceğiz. Terminale girmeniz gereken satırların tümü `$` ile başlar. `$`
> karakterini yazmanıza gerek yok; o, her komutun başlangıcını göstermek için
> görüntülenen komut satırı istemidir. `$` ile başlamayan satırlar genellikle
> önceki komutun çıktısını gösterir. Ayrıca PowerShell'e özgü örnekler `$`
> yerine `>` kullanır.

### Linux veya macOS'ta `rustup` Kurulumu

Linux veya macOS kullanıyorsanız bir terminal açın ve şu komutu girin:

```console
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Bu komut bir betiği indirir ve Rust'ın en son kararlı sürümünü kuran `rustup`
aracının kurulumunu başlatır. Şifreniz istenebilir. Kurulum başarılı olursa
aşağıdaki satır görünecektir:

```text
Rust is installed now. Great!
```

Ayrıca bir _linker_'a ihtiyacınız olacak. Linker, Rust'ın derlenmiş çıktılarını
tek bir dosyada birleştirmek için kullandığı bir programdır. Muhtemelen zaten
birine sahipsiniz. Linker hataları alırsanız, genellikle bir linker içeren bir C
derleyicisi kurmalısınız. C derleyicisi de kullanışlıdır çünkü bazı yaygın Rust
paketleri C koduna bağımlıdır ve bir C derleyicisine ihtiyaç duyar.

macOS'ta şu komutu çalıştırarak bir C derleyicisi edinebilirsiniz:

```console
$ xcode-select --install
```

Linux kullanıcıları, dağıtımlarının belgelerine göre genellikle GCC veya Clang
kurmalıdır. Örneğin, Ubuntu kullanıyorsanız `build-essential` paketini
kurabilirsiniz.

### Windows'ta `rustup` Kurulumu

Windows'ta [https://www.rust-lang.org/tools/install][install]<!-- ignore -->
adresine gidin ve Rust'ı kurma talimatlarını izleyin. Kurulumun bir noktasında
Visual Studio'yu kurmanız istenecek. Bu, programları derlemek için gereken
linker'ı ve yerel kütüphaneleri sağlar. Bu adımda daha fazla yardıma ihtiyacınız
olursa [https://rust-lang.github.io/rustup/installation/windows-msvc.html][msvc]<!--
ignore --> adresine bakın.

Bu kitabın geri kalanı hem _cmd.exe_ hem de PowerShell'de çalışan komutlar
kullanır. Belirli farklılıklar varsa hangisini kullanacağınızı açıklayacağız.

### Sorun Giderme

Rust'ın doğru kurulup kurulmadığını kontrol etmek için bir kabuk açın ve şu
satırı girin:

```console
$ rustc --version
```

Yayınlanmış en son kararlı sürümün sürüm numarasını, commit hash'ini ve commit
tarihini şu biçimde görmelisiniz:

```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

Bu bilgiyi görüyorsanız Rust'ı başarıyla kurmuşsunuz demektir! Bu bilgiyi
göremiyorsanız Rust'ın `%PATH%` sistem değişkeninizde olup olmadığını şu şekilde
kontrol edin.

Windows CMD'de şunu kullanın:

```console
> echo %PATH%
```

PowerShell'de şunu kullanın:

```powershell
> echo $env:Path
```

Linux ve macOS'ta şunu kullanın:

```console
$ echo $PATH
```

Her şey doğruysa ve Rust hâlâ çalışmıyorsa yardım alabileceğiniz birçok yer
var. Diğer Rustacean'larla (kendimize taktığımız şakacı bir takma ad) nasıl
iletişime geçeceğinizi [topluluk sayfasında][community] öğrenin.

### Güncelleme ve Kaldırma

`rustup` ile Rust kurulduktan sonra, yeni yayınlanan bir sürüme güncelleme
yapmak kolaydır. Kabuğunuzdan şu güncelleme betiğini çalıştırın:

```console
$ rustup update
```

Rust'ı ve `rustup`'ı kaldırmak için kabuğunuzdan şu kaldırma betiğini
çalıştırın:

```console
$ rustup self uninstall
```

<!-- Old headings. Do not remove or links may break. -->
<a id="local-documentation"></a>

### Yerel Belgeleri Okuma

Rust kurulumu, belgelerin yerel bir kopyasını da içerir; böylece onları
çevrimdışı okuyabilirsiniz. Yerel belgeleri tarayıcınızda açmak için
`rustup doc` komutunu çalıştırın.

Standart kütüphane tarafından sağlanan bir türün veya fonksiyonun ne yaptığından
ya da nasıl kullanılacağından emin değilseniz, öğrenmek için uygulama
programlama arayüzü (API) belgelerini kullanın!

<!-- Old headings. Do not remove or links may break. -->
<a id="text-editors-and-integrated-development-environments"></a>

### Metin Editörleri ve IDE'leri Kullanma

Bu kitap, Rust kodu yazmak için hangi araçları kullandığınız hakkında hiçbir
varsayımda bulunmaz. Hemen hemen her metin editörü işi görür! Ancak birçok metin
editörü ve tümleşik geliştirme ortamının (IDE) Rust için yerleşik desteği
vardır. Rust web sitesindeki [araçlar sayfasında][tools] birçok editörün ve
IDE'nin güncel bir listesini her zaman bulabilirsiniz.

### Bu Kitapla Çevrimdışı Çalışma

Birçok örnekte standart kütüphanenin ötesinde Rust paketleri kullanacağız. Bu
örnekler üzerinde çalışmak için ya internet bağlantınızın olması ya da bu
bağımlılıkları önceden indirmiş olmanız gerekir. Bağımlılıkları önceden
indirmek için aşağıdaki komutları çalıştırabilirsiniz. (`cargo`'nun ne olduğunu
ve bu komutların her birinin ne yaptığını daha sonra ayrıntılı olarak
açıklayacağız.)

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:

* ch02-00-guessing-game-tutorial.md
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
* ch14-03-cargo-workspaces.md
-->

```console
$ cargo new get-dependencies
$ cd get-dependencies
$ cargo add rand@0.10.1 trpl@0.2.0
```

Bu, bu paketlerin indirmelerini önbelleğe alır, böylece daha sonra onları tekrar
indirmeniz gerekmez. Bu komutu çalıştırdıktan sonra `get-dependencies`
klasörünü tutmanız gerekmez. Bu komutu çalıştırdıysanız, kitabın geri
kalanındaki tüm `cargo` komutlarında ağa erişmeye çalışmak yerine bu önbelleğe
alınmış sürümleri kullanmak için `--offline` bayrağını kullanabilirsiniz.

[otherinstall]: https://forge.rust-lang.org/infra/other-installation-methods.html
[install]: https://www.rust-lang.org/tools/install
[msvc]: https://rust-lang.github.io/rustup/installation/windows-msvc.html
[community]: https://www.rust-lang.org/community
[tools]: https://www.rust-lang.org/tools
