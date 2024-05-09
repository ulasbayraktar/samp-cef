## Yapı
- `cef.asi` dosyası, oyunun kök klasöründe bulunmalıdır (`loader.dll` olarak derlenmiştir).
- Tüm içeriğiyle birlikte `cef` klasörü de orada bulunmalıdır.
- Ayrıca, `My Documents/GTA San Andreas User Files/CEF/` konumunda `CEF` adında bir klasör oluşturulur. Bu klasör, Chromium'un doğru çalışması için gerekli olan önbellek, çerez dosyaları ve diğer şeyleri içerir.
- `gta_sa.exe`
- `cef.asi`
- `cef/`
    - `client.dll`
    - `libcef.dll`
    - `renderer.exe`
    - vs...

## Kullanım İpuçları ve Bazı Sınırlamalar
- İdeal olarak, tüm arabirimlere sahip tek bir tarayıcı olmalıdır. Farklı işlemler için yeni tarayıcılar oluşturmak yerine yerleşik olay sistemini kullanın.
- Eğer istemci eklentileri kullanıyorsa ve göreceli yolları kullanıyorsa, muhtemelen bozulacak ve yanlış çalışacaktır. Ne yazık ki, başlatma anında mevcut dizin başka bir iş parçacığında değişir. Örnek olarak: CLEO kütüphanesi, `cleo.log` adında bir günlük dosyası oluşturabilir ve `cef` klasöründe `cleo_text` ve `cleo_saves` gibi klasörler oluşturabilir. Doğru çalışma için mevcut yürütülebilir dosyanın yolunu belirlemek daha iyidir (`gta_sa.exe`).

## Pawn API

`cef_create_browser(player_id, browser_id, const url[], hidden, focused)`

Belirtilen oyuncu için bir tarayıcı oluşturur.

`cef_destroy_browser(player_id, browser_id)`

Tarayıcıyı yok eder.

`cef_hide_browser(player_id, browser_id, hide)`

Tarayıcıyı gizler.

`cef_emit_event(player_id, const event_name[], args…)`

İstemci üzerinde bir olay tetikler. Desteklenen argüman türleri: `string`, `integer`, `float`.

`cef_subscribe(const event_name[], const callback[])`

İstemciden bir olaya abone olur. Geri çağırma fonksiyonu imzası: `Callback(player_id, const arguments[])`

`cef_player_has_plugin(player_id)`

İstemcinin eklentiye sahip olup olmadığını kontrol eder.

`cef_create_ext_browser(player_id, browser_id, const texture[], const url[], scale)`

Belirtilen bir metinüründe nesnelerde görüntüleneceği belirtilen bir tarayıcı oluşturur. `scale` parametresi, standart metni kaç kat büyüteceğini belirtir. Örneğin, standart metnin bir boyutu varsa `250x30`, parametre 5 birim olarak ayarlanırsa, boyutu `1250x150` olur.

`cef_append_to_object(player_id, browser_id, object_id)`

Belirtilen nesnenin metnini istemci tarafında tarayıcı görüntüsüyle değiştirir. Tarayıcı, `cef_create_ext_browser` kullanılarak oluşturulmalı ve oluşturulurken belirtilen metin, doğru görüntüleme için mevcut olmalıdır.

`cef_remove_from_object(player_id, browser_id, object_id)`

Nesnenin orijinal metnini geri getirir.

`cef_toggle_dev_tools(player_id, browser_id, enabled)`

Geliştirici araçlarını etkinleştirir/devre dışı bırakır.

`native cef_set_audio_settings(player_id, browser_id, Float:max_distance, Float:reference_distance)`

Nesne üzerinde tarayıcı için maksimum işitilebilir mesafeyi ayarlar. `reference_distance`, maksimum ses seviyesine ulaşılacak mesafedir ve ardından ses, `max_distance` ile 0 arasında azalır.

`cef_focus_browser(player_id, browser_id, focused)`

Tarayıcıyı odaklanmış yapar. Öne çıkar, tüm klavye ve fare olaylarını alır. Tarayıcı oluşturulurken `focused = true` argümanını geçmekle aynıdır.

`cef_always_listen_keys(player_id, browser_id, listen)`

Tarayıcının arka planda klavye girişi almasını sağlar, hatta tarayıcının odaklanmadığı veya gizlenmediği durumlarda bile. Bu, JavaScript kodunda klavye olaylarına sürekli abone olmak için kullanılmasına izin verir. Örneğin, bir tuşa basarak arayüzü açabilir/kapatabilirsiniz (`window.addEventListener("keyup")`).

`cef_load_url(player_id, browser_id, const url[])`

Belirtilen URL'yi verilen tarayıcı için yükler. Tarayıcıyı yeniden oluşturmaktan daha hızlıdır.

### Ayrıca, eklentide iki yerleşik olay bulunmaktadır:

`forward OnCefBrowserCreated(player_id, browser_id, status_code)`

İstemci, sunucu/eklenti tarafından isteğe bağlı olarak bir tarayıcı oluşturduğunda çağrılır. `status_code` değeri, başarısız olası oluşturma için 0 veya bir HTTP kodu (200, 404, vb.) olabilir.

`forward OnCefInitialize(player_id, success)`

İstemcinin CEF sunucusuna bağlandıktan sonra veya zaman aşımı gerçekleştiğinde çağrılır. Yaklaşık olarak, `cef_player_has_plugin` kontrolünü el ile gerçekleştirir.

## Web API

Tarayıcılara yönetimlerini sağlamak için kendi API'ları da bulunmaktadır.

`cef.set_focus(focused)`

Tarayıcıya odaklanır, böylece tüm diğer pencerelerin üstünde olabilir ve klavye ve fare girişi alabilir.

`cef.on(event_name, callback)`

Tarayıcıdan/diğer eklentilerden bir olaya abone olur.

`cef.off(event_name, callback)`

ŞU ANDA UYGULANMAMIŞ
Bir olaydan aboneliği kaldırır. Bu işlevi kullanmak için, olaya abone olurken belirtilen geri çağırma fonksiyonunu içeren bir değişkeni geçirmeniz gerekir.

`cef.hide(hide)`

Tarayıcıyı gizler ve sesini kapatır.

`cef.emit(event_name, args…)`

Belirtilen argümanlarla sunucuda/harici eklentilerde bir olayı tetikler. Nesne ve fonksiyonlar hariç tüm tipleri destekler. Not: eklentilerde, tüm tipler normal olarak kullanılabilir, ancak sunucuda tüm argümanlar boşluklarla ayrılmış tek bir dizeye dönüştürülür.

## C API

Çalışmıyor, üzgünüm.

Çalışan örnek `cef-interface` içinde, ayrıca API `cef-api` ve `client/external.rs` içindedir.

```C++
    #include <cstdint>
    
    // Olay geri çağırımlarının işlemin devamını keser. Ayrıca, sunucuya gönderilmez.
    static const int EXTERNAL_BREAK = 1;
    // İşleme devam et. Kimse kesmediyse, sunucuya gönderilecektir.
    static const int EXTERNAL_CONTINUE = 0;
    
    using BrowserReadyCallback = void(*)(uint32_t);
    using EventCallback = int(*)(const char*, cef_list_value_t*);
    
    extern "C" {
        // Oyunda bir tarayıcının varlığını kontrol eder.
        bool cef_browser_exists(uint32_t browser);
        // Tarayıcının oluşturulup web sitesinin yüklendiği kontrol eder.
        bool cef_browser_ready(uint32_t browser);
        // Belirtilen parametrelerle bir tarayıcı oluşturur. Bu işlev asenkroniktir, tarayıcı hemen oluşturulmaz.
        void cef_create_browser(uint32_t id, const char *url, bool hidden, bool focused);
        // İstemci içinde bir CefListValue oluşturur.
        cef_list_value_t *cef_create_list();
        // İstemci tarafındaki tarayıcıyı yok eder.
        void cef_destroy_browser(uint32_t id);
        // Tarayıcıda bir olayı tetikler.
        void cef_emit_event(const char *event, cef_list_value_t *list);
        // Girişi tarayıcıya odaklar ve üste getirir, ayrıca klavye ve fare girişi alır.
        void cef_focus_browser(uint32_t id, bool focus);
        // Oyun penceresinin aktif olup olmadığını kontrol eder.
        bool cef_gta_window_active();
        // Tarayıcıyı gizler.
        void cef_hide_browser(uint32_t id, bool hide);
        // Belirli bir tarayıcı için girişin mevcut olup olmadığını kontrol eder.
        bool cef_input_available(uint32_t browser);
        // Tam tarayıcı oluşturma olayına abone olur.
        void cef_on_browser_ready(uint32_t browser, BrowserReadyCallback callback);
        bool cef_ready();
        // Tarayıcı olaylarına abone olur.
        void cef_subscribe(const char *event, EventCallback callback);
        // Tarayıcıya odaklanma girişiminde bulunur. `cef_input_available` + `cef_focus_browser` çiftine benzer,
        // ancak bu iki işlev arasında biri odaklanmışsa biri tarafından alınabilir. Bu işlev atomiktir, böylece kontrol edip hemen odaklanabilir,
        // bu süreçte başka kimse odaklanamaz.
        bool cef_try_focus_browser(uint32_t browser);
    }
```

## Kullanım Talimatları

### Açıklama
Bir tarayıcı iki yerden oluşturulabilir: oyun modundan ve eklentilerden.

Tarayıcının iki ek durumu daha vardır: `hidden` ve `focused`. İlk durum, tarayıcının oyuncunun ekranında görünüp görünmeyeceğini kontrol eder. İkinci durum daha karmaşıktır: eğer tarayıcı odaklanmışsa (`focused = true`), oyuncunun kamerası donar, bir imleç görünür ve klavye ve fare girişi doğrudan tarayıcıya gider, GTA ve SA:MP'yi atlayarak (F8 tuşu gibi bazı işlevler hariç). Oyuncu bu durumdan kendi başına çıkamaz; bunu tarayıcı arayüz kodunda kolaylaştırmalısınız. Örneğin, ESC tuşuna basılmasını dinleyebilir ve basıldığında `cef.set_focus(false)` çağrısını yapabilirsiniz.

Başka bir deyişle, bir websitesini youtube.com gibi bir yerde açtığınızda, oyunu kapatmadan veya bir tarayıcıyı modda silmeden asla çıkamazsınız.

Bu iki durum birbirinden tamamen bağımsızdır, yani tarayıcı `gizli = false` olabilir ancak aynı anda `focused = false` olabilir. Bu durumda, tarayıcı gösterilir ancak giriş erişimi olmaz ve oyuncu oyun içinde serbestçe eylemler gerçekleştirebilir.

Oyun modundan etkileşim
Kısacası: oyun modu yalnızca birkaç yerel işlevi kullanmalıdır (tarayıcıları oluşturma/silme, tarayıcıda olayları tetikleme ve onlara abone olma).
