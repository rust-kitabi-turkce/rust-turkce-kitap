# Katkıda Bulunma Rehberi

Bu projeye katkıda bulunduğunuz için teşekkürler! 🎉

Bu rehber, Resmi Rust Kitabı'nın Türkçe çevirisine katkıda bulunmak isteyen herkes için hazırlanmıştır. Issue açmadan veya Pull Request (PR) göndermeden önce lütfen bu rehberi dikkatlice okuyun.

## İçindekiler

- [Proje Yapısı](#proje-yapısı)
- [Issue Açma Kuralları](#issue-açma-kuralları)
- [Pull Request (PR) Kuralları](#pull-request-pr-kuralları)
- [Çeviri Kuralları](#çeviri-kuralları)
- [Terminoloji](#terminoloji)
- [Kod Örnekleri](#kod-örnekleri)
- [Build ve Test](#build-ve-test)
- [İletişim](#iletişim)

## Proje Yapısı

Projenin temel yapısı şu şekildedir:

| Klasör / Dosya | Açıklama |
|---|---|
| `original/src/*.md` | Orijinal İngilizce kaynak (**ASLA değiştirmeyin**) |
| `src/*.md` | Türkçe çeviri (buraya yazılır, buradan build edilir) |
| `listings/` | Kod örnekleri (sadece string/yorum çevirisi yapılır) |
| `packages/` | mdBook preprocessor'ları (**ASLA değiştirmeyin**) |
| `glossary.json` | Çeviri sözlüğü |
| `ceviri-durumu.md` | Çevirisi tamamlanan bölümlerin listesi |
| `book/` | Build çıktısı (geçici, gitignore'da) |

## Issue Açma Kuralları

### Issue Açmadan Önce

1. **Arama yapın**: Aynı konuda daha önce issue açılmış mı diye kontrol edin. Açılmışsa yeni issue açmak yerine mevcut issue'ya yorum yapın.
2. **Başlığı netleştirin**: Başlık, sorunun ne olduğunu kısa ve öz şekilde anlatmalıdır.
3. **Şablon kullanın**: Issue şablonunu doldurun (varsa).

### Issue Başlığı Formatı

Başlık, sorunun türünü ve ilgili bölümü belirtmelidir:

```
[Bölüm Adı] Sorunun kısa açıklaması
```

Örnekler:
- `[Bölüm 4.1] "sahiplik" terimi ilk kullanımda parantez içinde yazılmamış`
- `[Bölüm 2] Tahmin oyunundaki çıktı metinleri İngilizce kalmış`
- `[Genel] glossary.json'a "borrow" terimi eklenmeli`

### Issue Türleri

Aşağıdaki türlerde issue açabilirsiniz:

- **Çeviri hatası**: Yanlış çeviri, eksik çeviri, tutarsız terminoloji
- **Yazım hatası**: Türkçe yazım kurallarına uymayan ifadeler
- **Kod hatası**: Kod örneklerinde çalışmayan veya hatalı kod
- **Terminoloji önerisi**: `glossary.json`'a yeni terim ekleme önerisi
- **Yapısal sorun**: Build hatası, bağlantı hatası, biçimlendirme sorunu

### Issue Açarken Belirtilmesi Gerekenler

- Sorunun bulunduğu dosya adı ve satır numarası (mümkünse)
- Sorunun açıklaması ve neden sorun olduğu
- Varsa önerilen düzeltme
- Ekran görüntüsü (görsel bir sorunsa)

## Pull Request (PR) Kuralları

### PR Açmadan Önce

1. **Issue ile ilişkilendirin**: PR'ınız bir issue'yu çözüyorsa, PR açıklamasında `Closes #issue_numarası` şeklinde belirtin.
2. **Küçük ve odaklı PR'lar**: Tek bir PR'da tek bir konuya odaklanın. Birden fazla bölümü tek PR'da değiştirmeyin.
3. **Branch oluşturun**: Ana branch'e (`main`) doğrudan commit yapmayın. Açıklayıcı bir isimle yeni bir branch oluşturun:
   - `ceviri/ch04-01-sahiplik`
   - `duzeltme/ch02-cikti-metinleri`
   - `terminoloji/glossary-ekleme`
4. **Değişiklikleri test edin**: Değişikliklerinizi göndermeden önce `mdbook build` komutuyla build'in başarılı olduğunu doğrulayın.

### PR Açıklaması

PR açıklamasında şunları belirtin:

- Ne değiştirdiğiniz ve neden
- Hangi issue'yu çözdüğü (varsa)
- Yaptığınız değişikliklerin özeti
- Test ettiyseniz build sonucu

### PR Kontrol Listesi

PR göndermeden önce şunları kontrol edin:

- [ ] `original/src/` klasöründe değişiklik yapılmadı
- [ ] `packages/` klasöründe değişiklik yapılmadı
- [ ] `glossary.json`'a uygun terminoloji kullanıldı
- [ ] Kod örneklerinde kod yapısı değiştirilmedi (sadece string/yorum çevirisi)
- [ ] `src/SUMMARY.md`'deki dosya adları ve sıralama değiştirilmedi
- [ ] Çevirisi tamamlanan bölüm `ceviri-durumu.md`'ye eklendi
- [ ] `mdbook build` komutu başarıyla çalıştı

### PR Sonrası

- İnceleme sürecinde yorumlara yanıt verin
- İstenen değişiklikleri yapın ve force-push yerine yeni commit'lerle güncelleyin
- Çakışma (conflict) varsa çözün

## Çeviri Kuralları

### Çevrilecekler

- Düz anlatım metinleri
- Bölüm başlıkları (`#`, `##`)
- Tablo metinleri
- `<img alt="...">` metinleri
- `src/SUMMARY.md` başlıkları (Türkçe, ama aynı sıra ve dosya adları)
- Kod örneklerindeki string literal'ler ve yorumlar

### ASLA Çevrilmeyecekler / Değiştirilmeyecekler

- ` ``` ` ile çevrili Rust kodunun **yapısı** (satırlar, ifadeler, fonksiyon çağrıları, operatörler)
- `{{#rustdoc_include}}`, `{{#include}}`, `{{#playground}}` satırları
- Teknik adlar: `cargo`, `rustc`, `rustfmt`, `mdbook`, `Cargo.toml`, `main.rs` vb.
- Değişken adları, fonksiyon adları, tür adları, struct/enum/trait adları, metot adları, alan adları
- `let`, `fn`, `impl`, `match` gibi anahtar kelimeler
- Teknik terimler (kod içinde geçse bile): `ownership`, `borrow`, `lifetime`, `trait`, `enum` vb.
- İngilizce `CODE_x` / `Listing x-x` etiketleri
- `<span class="filename">...</span>` içeriği (dosya adı kalır)
- URL'ler ve bağlantıların hedef kısmı (anchor'ların görünen metni çevrilir, hedef DEĞİŞMEZ)
- `<img>` etiketlerinin `src`/`class` öznitelikleri
- Derleyici uyarıları ve hata çıktıları (`error[E0308]` gibi)

### Kod İçindeki String ve Yorum Çevirisi

Kod örneklerinde **kodun amacını/çalışma mantığını DEĞİŞTİRMEDEN** şunlar Türkçeye çevrilir:

- String literal'ler ve bunların basıldığı çıktılar
- Yorum satırları: `// ...`, `/* ... */`, `/// ...`
- `panic!`, `assert!`, `assert_eq!`, `unwrap_or_else`, `expect`, `eprintln!` gibi hata/çıktı mesajlarındaki kullanıcıya görünen metinler
- Çevrilen string'lerin karşılığı olan komut çıktıları (`output.txt` dosyaları) DA çevrilir ve kodla TUTARLI olur

**ASLA çevrilmeyen (kod davranışını etkilediği için):**

- Değişken/tür/fonksiyon/alan adları, `let`, `fn`, `impl`, `match` gibi anahtar kelimeler
- String **karşılaştırmaları** ve girdi olarak kullanılan string'ler (ör. `if guess == "quit"`, `std::env::args`, dosya adları)
- İçinde string geçen ama anlamlı olan sabitler (`const MESSAGE: &str = ...`)
- Gerçek derleyici/hata çıktıları İngilizce kalır

## Terminoloji

- **`glossary.json`'a %100 uyun.** Terim bilinmiyorsa sözlüğe uygun girdi eklemeyin — önce issue açarak sorun.
- İlk kullanım: `sahiplik (ownership)` -> sonraki kullanımlarda yalnızca `sahiplik`
- Kod içinde identifier olarak geçen terimler (ör. `ownership` değişken adı): olduğu gibi kalır. Kod yorumu/string'inde geçiyorsa sözlükteki Türkçe karşılığıyla çevrilir.

## Build ve Test

Lokal olarak build almak için:

```bash
mdbook build
```

Build çıktısı `book/` klasörüne oluşturulur. Build sırasında hata alırsanız, hatayı düzeltmeden PR göndermeyin.

## İletişim

- Kitap adresi: https://rust-kitabi-turkce.github.io/rust-turkce-kitap/
- GitHub reposu: https://github.com/rust-kitabi-turkce/rust-turkce-kitap

Sorularınız varsa issue açmaktan çekinmeyin. İyi çeviriler! 🦀