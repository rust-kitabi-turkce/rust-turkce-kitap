## Merhaba, Dünya!

Rust'ı kurduğunuza göre, ilk Rust programınızı yazmanın zamanı geldi. Yeni bir
dil öğrenirken ekrana `Merhaba, dünya!` metnini yazdıran küçük bir program
yazmak gelenektir, bu yüzden biz de burada aynısını yapacağız!

> Not: Bu kitap komut satırına temel düzeyde aşinalık varsayar. Rust, düzenleme
> veya araç seçiminiz ya da kodunuzun nerede durulacağı konusunda hiçbir özel
> talepte bulunmaz; bu yüzden komut satırı yerine bir IDE kullanmayı tercih
> ediyorsanız, en sevdiğiniz IDE'yi kullanmaktan çekinmeyin. Pek çok IDE'nin
> artık bir dereceye kadar Rust desteği vardır; ayrıntılar için IDE'nin
> belgelerine bakın. Rust ekibi, `rust-analyzer` aracılığıyla harika bir IDE
> desteği sağlamaya odaklanmaktadır. Daha fazla ayrıntı için
> [Ek D'ye][devtools]<!-- ignore --> bakın.

<!-- Old headings. Do not remove or links may break. -->
<a id="creating-a-project-directory"></a>

### Proje Dizini Düzeni

Rust kodunuzu saklamak için bir dizin oluşturarak başlayacaksınız. Kodunuzun
nerede durduğu Rust için önemli değildir, ancak bu kitaptaki alıştırmalar ve
projeler için, ana dizininizde bir _projects_ dizini oluşturmanızı ve tüm
projelerinizi orada tutmanızı öneriyoruz.

Bir terminal açın ve bir _projects_ dizini ile _projects_ dizini içinde
"Merhaba, dünya!" projesi için bir dizin oluşturmak üzere şu komutları girin.

Linux, macOS ve Windows'taki PowerShell için bunu girin:

```console
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world
```

Windows CMD için bunu girin:

```cmd
> mkdir "%USERPROFILE%\projects"
> cd /d "%USERPROFILE%\projects"
> mkdir hello_world
> cd hello_world
```

<!-- Old headings. Do not remove or links may break. -->
<a id="writing-and-running-a-rust-program"></a>

### Rust Programının Temelleri

Ardından, yeni bir kaynak dosyası oluşturun ve buna _main.rs_ adını verin. Rust
dosyaları her zaman _.rs_ uzantısıyla biter. Dosya adınızda birden fazla kelime
kullanıyorsanız, bunları birbirinden ayırmak için genelde alt çizgi kullanılır.
Örneğin, _helloworld.rs_ yerine _hello_world.rs_ kullanın.

Şimdi az önce oluşturduğunuz _main.rs_ dosyasını açın ve Liste 1-1'deki kodu
girin.

<Listing number="1-1" file-name="main.rs" caption="`Merhaba, dünya!` yazdıran bir program">

```rust
fn main() {
    println!("Merhaba, dünya!");
}
```

</Listing>

Dosyayı kaydedin ve _~/projects/hello_world_ dizinindeki terminal pencerenize
dönün. Linux veya macOS'ta dosyayı derlemek ve çalıştırmak için şu komutları
girin:

```console
$ rustc main.rs
$ ./main
Merhaba, dünya!
```

Windows'ta `./main` yerine `.\main` komutunu girin:

```powershell
> rustc main.rs
> .\main
Merhaba, dünya!
```

Hangi işletim sistemini kullanırsanız kullanın, `Merhaba, dünya!` metni
terminale yazdırılmalıdır. Bu çıktıyı göremiyorsanız, yardım almanın yolları
için Kurulum bölümündeki ["Sorun Giderme"][troubleshooting]<!-- ignore -->
bölümüne geri dönün.

`Merhaba, dünya!` yazdırıldıysa, tebrikler! Resmi olarak bir Rust programı
yazdınız. Bu, sizi bir Rust programcısı yapar— hoş geldiniz!

<!-- Old headings. Do not remove or links may break. -->
<a id="anatomy-of-a-rust-program"></a>

### Rust Programının Anatomisi

Bu "Merhaba, dünya!" programını ayrıntılı olarak inceleyelim. İşte bulmacanın
ilk parçası:

```rust
fn main() {

}
```

Bu satırlar `main` adlı bir fonksiyonu tanımlar. `main` fonksiyonu özeldir: her
çalıştırılabilir Rust programında her zaman ilk çalışan koddur. Burada ilk
satır, parametresi olmayan ve hiçbir şey döndürmeyen `main` adlı bir fonksiyonu
bildirir. Parametreler olsaydı, parantezlerin (`()`) içine girerlerdi.

Fonksiyon gövdesi `{}` ile sarılır. Rust, tüm fonksiyon gövdelerinin etrafında
süslü parantezler ister. Açılış süslü parantezini fonksiyon bildiriminin aynı
satırına yerleştirmek ve araya bir boşluk eklemek iyi bir üsluptur.

> Not: Rust projeleri arasında standart bir stile bağlı kalmak istiyorsanız,
> kodunuzu belirli bir stilde biçimlendirmek için `rustfmt` adında otomatik bir
> biçimlendirici kullanabilirsiniz (`rustfmt` hakkında daha fazla bilgi için
> [Ek D'ye][devtools]<!-- ignore --> bakın). Rust ekibi bu aracı, `rustc`
> ile birlikte standart Rust dağıtımına dahil etmiştir; bu yüzden artık
> bilgisayarınızda kurulu olmalıdır!

`main` fonksiyonunun gövdesi şu kodu içerir:

```rust
println!("Merhaba, dünya!");
```

Bu satır, bu küçük programdaki tüm işi yapar: ekrana metin yazdırır. Burada
dikkat edilmesi gereken üç önemli ayrıntı vardır.

İlk olarak, `println!` bir Rust makrosunu çağırır. Bunun yerine bir fonksiyon
çağırsaydı, `println` (ünlem işareti olmadan) şeklinde girilirdi. Rust makroları,
Rust sözdizimini genişleten kod üreten kod yazmanın bir yoludur ve bunları
[20. Bölümde][ch20-macros]<!-- ignore --> daha ayrıntılı ele alacağız. Şimdilik
bilmeniz gereken, bir `!` kullanmanın normal bir fonksiyon yerine bir makro
çağırdığınız anlamına geldiği ve makroların her zaman fonksiyonlarla aynı
kurallara uymadığıdır.

İkincisi, `"Merhaba, dünya!"` metnini görürsünüz. Bu metni `println!`'a bir
argüman olarak geçiririz ve metin ekrana yazdırılır.

Üçüncüsü, satırı noktalı virgülle (`;`) sonlandırırız; bu, bu ifadenin bittiğini
ve bir sonrakinin başlamaya hazır olduğunu gösterir. Rust kodunun çoğu satırı
noktalı virgülle biter.

<!-- Old headings. Do not remove or links may break. -->
<a id="compiling-and-running-are-separate-steps"></a>

### Derleme ve Çalıştırma

Az önce yeni oluşturulmuş bir program çalıştırdınız, bu yüzden sürecin her
adımını inceleyelim.

Bir Rust programını çalıştırmadan önce, `rustc` komutunu girip kaynak
dosyanızın adını ona geçirerek Rust derleyicisiyle derlemeniz gerekir, şöyle:

```console
$ rustc main.rs
```

Bir C veya C++ geçmişiniz varsa, bunun `gcc` veya `clang`a benzediğini fark
edeceksiniz. Başarılı bir şekilde derledikten sonra Rust, ikili (binary)
çalıştırılabilir bir dosya üretir.

Linux, macOS ve Windows'taki PowerShell'de, kabuğunuzda `ls` komutunu girerek
çalıştırılabilir dosyayı görebilirsiniz:

```console
$ ls
main  main.rs
```

Linux ve macOS'ta iki dosya görürsünüz. Windows'taki PowerShell'de ise CMD
kullanırken göreceğiniz aynı üç dosyayı görürsünüz. Windows'ta CMD ile şunu
girersiniz:

```cmd
> dir /B %= the /B option says to only show the file names =%
main.exe
main.pdb
main.rs
```

Bu, _.rs_ uzantılı kaynak kodu dosyasını, çalıştırılabilir dosyayı (_main.exe_
Windows'ta, ancak diğer tüm platformlarda _main_) ve Windows kullanırken hata
ayıklama bilgisi içeren _.pdb_ uzantılı bir dosyayı gösterir. Buradan itibaren
_main_ veya _main.exe_ dosyasını şöyle çalıştırırsınız:

```console
$ ./main # veya Windows'ta .\main
```

_main.rs_ dosyanız "Merhaba, dünya!" programınızsa, bu satır terminalinize
`Merhaba, dünya!` yazdırır.

Ruby, Python veya JavaScript gibi dinamik bir dile daha aşinaysanız, bir
programı ayrı adımlar olarak derlemek ve çalıştırmak size alışılmadık
gelebilir. Rust, _önceden derlenen (ahead-of-time compiled)_ bir dildir; yani bir
programı derleyip çalıştırılabilir dosyayı başka birine verebilirsiniz ve o
kişi, Rust kurulu olmasa bile programı çalıştırabilir. Birine bir _.rb_, _.py_
veya _.js_ dosyası verirseniz, o kişinin sırasıyla bir Ruby, Python veya
JavaScript uygulamasının kurulu olması gerekir. Ancak bu dillerde programınızı
derlemek ve çalıştırmak için yalnızca bir komuta ihtiyacınız vardır. Dil
tasarımında her şey bir takastır.

Basit programlar için `rustc` ile derlemek yeterlidir, ancak projeniz
büyüdükçe, tüm seçenekleri yönetmek ve kodunuzu paylaşmayı kolaylaştırmak
isteyeceksiniz. Sırada, gerçek dünya Rust programları yazmanıza yardımcı olacak
Cargo aracıyla sizi tanıştıracağız.

[troubleshooting]: ch01-01-installation.html#sorun-giderme
[devtools]: appendix-04-useful-development-tools.html
[ch20-macros]: ch20-05-macros.html
