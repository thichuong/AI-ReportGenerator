//! Interface creation prompts

pub const CREATE_INTERFACE_PROMPT: &str = r#"# YÊU CẦU: TẠO GIAO DIỆN BÁO CÁO PHÂN TÍCH

## 1. Vai trò (Persona)

Bạn là một lập trình viên frontend và nhà thiết kế dữ liệu, có khả năng biến những báo cáo dày đặc chữ thành một giao diện web trực quan, sinh động và dễ hiểu.

## 2. Bối cảnh (Context)

Nhiệm vụ là tạo ra một bộ tệp (`HTML`, `CSS`, `JS`) để hiển thị một báo cáo phân tích chi tiết.

-   **Mục đích chính:** Nội dung của tệp `report.html` sẽ được tệp `main.js` tải động vào bên trong tệp `index.html`. Điều này giúp trang chính tải nhanh hơn và nội dung báo cáo có thể được cập nhật độc lập.
-   **Tệp `report.html`**: Chỉ chứa cấu trúc HTML của báo cáo, không bao gồm `<html>`, `<head>`, `<body>`.
-   **Tệp `report.css`**: Chứa các style tùy chỉnh cho báo cáo.
-   **Tệp `report.js`**: Chứa các hàm để vẽ biểu đồ, đồ thị.

## 3. Mục tiêu & Tệp cần tạo

Bạn PHẢI trả về 3 khối mã riêng biệt và đầy đủ, được bao bọc chính xác như sau:
1.  Một khối mã ````html`...````
2.  Một khối mã ````css`...````
3.  Một khối mã ````javascript`...````

### **TÓM TẮT VÀ NHẤN MẠNH:**
- **KHÔNG BỎ SÓT** thông tin quan trọng từ nội dung gốc
- **Trích xuất và nhấn mạnh** chi tiết kỹ thuật quan trọng
- **Làm nổi bật** tóm tắt, kết luận, điểm nhấn với HTML formatting
- **Ưu tiên bảng** (`<table>`) cho dữ liệu có cấu trúc

-   **CÁCH THỨC TRÌNH BÀY:**
    -   Trình bày thông tin một cách **rõ ràng, nhấn mạnh các chỉ số, con số**
    -   Sử dụng các list, thẻ `<strong>`, `<em>` để làm nổi bật các điểm chính
    -   **Ưu tiên sử dụng bảng** (`<table>`) cho các dữ liệu có cấu trúc thay vì đoạn văn
    -   **Tạo các section riêng biệt** cho từng chủ đề để tránh nhầm lẫn thông tin

-   **PHÂN TÍCH VÀ TỔ CHỨC:**
    -   Phân tích nội dung để tạo ra các thành phần giao diện phù hợp như thẻ (cards), biểu đồ, bảng biểu, danh sách, v.v.
    -   **Nhóm thông tin liên quan** vào cùng một card/section
    -   **Tạo hierarchy rõ ràng** với các mức tiêu đề từ h2 đến h4 tùy theo độ quan trọng

-   **ĐỒNG BỘ ID:** Các `id` của các element trong `report.html` (ví dụ: `<div id="fear-greed-gauge-container">`) PHẢI khớp chính xác với các `id` được sử dụng trong `report.js` (ví dụ: `document.getElementById('fear-greed-gauge-container')`). Sử dụng quy tắc đặt tên `kebab-case` cho ID để đảm bảo tính nhất quán.

## 5. Yêu cầu chi tiết

### 5.1. `report.html`

## CẤU TRÚC HTML YÊU CẦU:

### **1. Semantic Structure:**
```html
<section id="summary-section">
    <h2><i class="fas fa-chart-line"></i> Tóm tắt Điều hành</h2>
    <div class="report-card">
        <!-- Executive summary content -->
    </div>
</section>

<section id="market-analysis-section">
    <h2><i class="fab fa-bitcoin"></i> Phân tích Thị trường</h2>
    <div class="content-grid">
        <div class="report-card">
            <h3>Fear & Greed Index</h3>
            <div id="fear-greed-gauge-container"></div>
            <div id="fear-greed-text-analysis"></div>
        </div>
        <div class="report-card">
            <h3>Bitcoin Dominance</h3>
            <div id="btc-dominance-doughnut-container"></div>
            <div id="btc-dominance-text-analysis"></div>
        </div>
    </div>
</section>
```

### **2. Layout System - Smart Nested Grid:**

#### **Khi có 2-3 items trên 1 hàng:** Lưu ý class `content-grid` phải là con trực tiếp của `section` và không được nhỏ hơn class khác.
```html
<div class="content-grid">
    <div class="report-card">...</div>
    <div class="report-card">...</div>
    <div class="report-card">...</div>
</div>
```

#### **Khi cần chiếm full width:**
```html
<section id="market-analysis-section">
    <h2><i class="fab fa-bitcoin"></i> Phân tích Thị trường</h2>
    <div class="report-card">
        <h3>Fear & Greed Index</h3>
        <div id="fear-greed-gauge-container"></div>
        <div id="fear-greed-text-analysis"></div>
    </div>
    <div class="report-card">
        <!-- content -->
    </div>
</section>
``` 

-   **Trực quan hóa dữ liệu:**
    -   Đối với các chỉ số quan trọng (như RSI, Fear & Greed), hãy tạo các placeholder `<div>` với `id` rõ ràng để JavaScript có thể vẽ biểu đồ vào đó. Ví dụ: `<div id="rsi-gauge-container"></div>`.
    -   Đối với các bảng dữ liệu, hãy biến chúng thành các bảng HTML (`<table>`) gọn gàng và đặt chúng trong một container cho phép cuộn ngang nếu cần (`<div class="table-container">`).
    -   Đối với các chỉ số như Tỷ lệ Thống trị của Bitcoin, hãy tạo các placeholder `<div>` với `id` rõ ràng để JavaScript có thể vẽ biểu đồ vào đó. Ví dụ: `<div id="btc-dominance-doughnut-container"></div>`.
-   **Tối ưu hóa cho Thiết bị Di động:** 
    -   Tạo phiên bản hiển thị dạng "card" cho mỗi hàng của bảng trên di động thay vì cuộn ngang, giúp thông tin dễ đọc hơn.
    -   **Căn giữa hoàn hảo:** Tất cả các `.report-card` phải được căn giữa một cách hoàn hảo trên thiết bị di động, sử dụng Flexbox với `align-items: center` và `justify-content: center`.
### **3 Data Tables:**
```html
<div class="table-container">
    <table class="crypto-table">
        <thead>
            <tr>
                <th>Rank</th>
                <th>Coin</th>
                <th>Price</th>
                <th>24h Change</th>
                <th>Market Cap</th>
                <th>Volume</th>
            </tr>
        </thead>
        <tbody>
            <tr>
                <td data-label="Rank">#1</td>
                <td data-label="Coin">Bitcoin (BTC)</td>
                <td data-label="Price">$43,250.00</td>
                <td data-label="24h Change">+2.45%</td>
                <td data-label="Market Cap">$847.2B</td>
                <td data-label="Volume">$15.2B</td>
            </tr>
            <tr>
                <td data-label="Rank">#2</td>
                <td data-label="Coin">Ethereum (ETH)</td>
                <td data-label="Price">$2,580.00</td>
                <td data-label="24h Change">-1.23%</td>
                <td data-label="Market Cap">$310.5B</td>
                <td data-label="Volume">$8.7B</td>
            </tr>
            <!-- More data rows... -->
        </tbody>
    </table>
</div>
```

**📱 MOBILE CARD LAYOUT:**
- **Desktop/Tablet**: Hiển thị bảng thông thường
- **Mobile (≤580px)**: Mỗi row hiển thị thành card riêng biệt
- **Required**: Mỗi `<td>` phải có attribute `data-label` để hiển thị tên cột trên mobile
- **Format**: `<td data-label="Tên Cột">Giá trị</td>`

**VÍ DỤ SỬ DỤNG:**
```html
<!-- Desktop: Table format | Mobile: Card format -->
<td data-label="Coin">Bitcoin (BTC)</td>
<td data-label="Price">$43,250.00</td>
<td data-label="24h Change" style="color: var(--color-gain);">+2.45%</td>
```

### 5.2. `report.css`
#### BỐI CẢNH & NHIỆM VỤ:
- **CSS CƠ BẢN ĐÃ CÓ**: Layout, cards, charts, responsive đã được hardcode trong `app/static/css/report.css`
- **NHIỆM VỤ CỦA BẠN**: Chỉ tạo **THEME COLORS** và **SPECIAL STYLING** cho nội dung cụ thể
- **CSS SCOPE**: Tất cả selectors phải trong `#report-container`
- **SỬ DỤNG MÀU TỰ DO**: Cho phép dùng màu tự do, nhưng đảm bảo tương thích với light theme và dark theme
- Tập trung vào màu sắc crypto, highlighting data, và visual accents

#### 🎨 CSS VARIABLES AVAILABLE:

#### NHỮNG GÌ ĐÃ CÓ SẴN (KHÔNG CẦN VIẾT LẠI):

#### ✅ **Layout System** (Đã có):
- Grid layouts, flexbox containers
- Responsive breakpoints  
- Card system và spacing
- Typography hierarchy (h1, h2, h3)

#### ✅ **Chart Containers** (Đã có):
- `.gauge-container`, `.doughnut-container`
- `.line-chart-container`, `.bar-chart-container`
- `.chart-container` - **KHÔNG TẠO CSS CHO CLASS NÀY**
- Chart responsive sizing

#### ✅ **Table System** (Đã có):
- Table layouts và responsive mobile conversion
- Basic table styling

#### ✅ **Interactive Elements** (Đã có):
- Hover effects, transitions
- Loading states, skeleton animations

#### FOCUS AREA - CHỈ VIẾT NHỮNG PHẦN NÀY:

##### **1. 🎨 CRYPTO THEME COLORS (SỬ DỤNG MÀU TỰ DO)**
```css
/* Theme-specific color applications */
#report-container .crypto-positive {
    color: #00ff00; /* Success green */
}

#report-container .crypto-negative {
    color: #ff0000; /* Danger red */
}

#report-container .bitcoin-accent {
    color: #f7931a; /* Bitcoin orange */
}

#report-container .ethereum-accent {
    color: #627eea; /* Ethereum blue */
}
```

##### **2. 📊 DATA VISUALIZATION COLORS (SỬ DỤNG MÀU TỰ DO)**
```css
/* Specific color coding for tables, stats */
#report-container td.positive { 
    color: #00ff00; 
    font-weight: 600; 
}

#report-container td.negative { 
    color: #ff0000; 
    font-weight: 600; 
}
```

##### **3. 🔥 SPECIAL CONTENT STYLING**
- **Fear & Greed levels** với màu tự do
- **Bull/Bear card styling** với background colors tự do
- **Support/Resistance levels** với color coding tự do
- **Price change indicators** sử dụng positive/negative colors
- **Volume spike highlighting** với accent colors

##### **4. 💡 CONTENT-SPECIFIC HIGHLIGHTS**
- Highlighting important metrics với màu tự do
- Color coding cho different crypto categories từ icon colors
- Special styling cho breaking news với accent colors
- Emphasis colors cho key insights với màu tự do

#### CRYPTO COLOR VARIABLES CÓ SẴN:

#### **Core Crypto Colors (Từ CSS Variables):**

#### **Market Sentiment Colors (Từ CSS Variables):**

#### **Technical Analysis Colors (Từ CSS Variables):**

#### CONTENT-SPECIFIC STYLING EXAMPLES (SỬ DỤNG MÀU TỰ DO):

#### OUTPUT YÊU CẦU:

#### **1. FOCUS CHỈ VÀO:**
- 🎨 Theme colors cho crypto elements (SỬ DỤNG MÀU TỰ DO)
- 📊 Data visualization color coding (SỬ DỤNG MÀU TỰ DO)
- 🔥 Special content highlighting (SỬ DỤNG MÀU TỰ DO)
- 💡 Content-specific styling (SỬ DỤNG MÀU TỰ DO)
- 🎯 Visual emphasis cho key metrics (SỬ DỤNG MÀU TỰ DO)

#### **2. KHÔNG VIẾT LẠI:**
- ❌ Layout systems (grid, flexbox)
- ❌ Card structures  
- ❌ Typography base styles
- ❌ Responsive breakpoints
- ❌ Chart containers (đặc biệt `.chart-container`)

#### **3. QUY TẮC QUAN TRỌNG:**
- ✅ **SỬ DỤNG MÀU TỰ DO**: Cho phép hardcode màu thay vì var(--variable-name), nhưng đảm bảo tương thích với light/dark theme
- ✅ **THAM KHẢO CSS VARIABLES AVAILABLE**: Có thể tham khảo nếu cần
- ✅ **SCOPE ĐÚNG**: Tất cả selector phải bắt đầu với `#report-container`
- ✅ **FOCUS THEME**: Chỉ tập trung vào colors và visual accents

#### **4. FORMAT:**
Chỉ trả về CSS code trong ```css``` block.
CSS phải focused và chỉ bao gồm theme colors + special styling sử dụng màu tự do.

---

**LƯU Ý**: Bạn đang bổ sung cho hệ thống CSS đã có sẵn, không phải viết từ đầu. Tập trung vào colors và visual accents để làm nổi bật nội dung crypto. **QUAN TRỌNG**: Sử dụng màu tự do, nhưng đảm bảo tương thích với light/dark theme.



### 5.3. `report.js`

-   **Thư viện đồ họa:** Sử dụng các hàm vẽ biểu đồ có sẵn trong tệp `chart.js`. Không cần viết lại logic vẽ.
-   **Đa dạng hóa biểu đồ:** Không chỉ giới hạn ở biểu đồ dạng đồng hồ đo (gauge). Hãy xem xét việc sử dụng các loại biểu đồ khác được cung cấp bởi `chart.js`.
-   **Hàm khởi tạo:** Cung cấp một hàm chính gọi là `initializeAllVisuals_report()`, để gọi và vẽ tất cả các biểu đồ cần thiết cho báo cáo.
-   **Hàm trong `report.js` thêm hậu tố:** Các hàm trong `report.js` nên có hậu tố `_report` để tránh trùng tên với các hàm trong `main.js` (ví dụ: `initializeAllVisuals_report`).
-   ** Mã nguồn phải sạch sẽ, không có lỗi console.**
-   **Tối ưu hóa cho Thiết bị Di động:** 
    -   Phóng to và thu nhỏ chart tùy theo màn hình di động và máy tính.
-   Chỉ vẽ chart không thêm các nội dung khác.
### 5.4. Quy ước về Hàm vẽ Biểu đồ (chart.js)

Tất cả các hàm vẽ biểu đồ đều nằm trong `chart.js`. Khi gọi các hàm này, hãy tuân thủ đúng cấu trúc tham số đầu vào.
-   Với chỉ số như RSI, Fear & Greed dùng GAUGE
-   Với chỉ số tỉ lệ như BTC.D dùng DOUGHNUT CHART

---
 #### TẠO BIỂU ĐỒ ĐỒNG HỒ (GAUGE)
 
 * @param `{HTMLElement}` container - **Đầu vào:** Element DOM để chứa biểu đồ.
 * @param `{number}` value - **Đầu vào:** Giá trị số hiện tại để hiển thị.
 * @param `{object}` config - **Đầu vào:** Đối tượng cấu hình.
 * @param `{number}` [config.min=0] - (Tùy chọn) Giá trị tối thiểu của thang đo.
 * @param `{number}` [config.max=100] - (Tùy chọn) Giá trị tối đa của thang đo.
 * @param `{Array<object>}` config.segments - Mảng các đối tượng. Mỗi object chứa:
 * - `{number}` limit: Giá trị giới hạn trên của đoạn.
 * - `{string}` color: Màu của đoạn (biến CSS hoặc mã màu).
 * - `{string}` label: Nhãn phân loại cho giá trị khi rơi vào đoạn này.
 * @returns `{void}` **Đầu ra:** Hàm này không trả về giá trị. Nó sẽ vẽ một biểu đồ SVG vào bên trong `container` được cung cấp.
 
`function createGauge(container, value, config) { /* ... */ }`

---
#### TẠO BIỂU ĐỒ ĐƯỜNG (LINE CHART)

 * @param `{HTMLElement}` container - **Đầu vào:** Element DOM để chứa biểu đồ.
 * @param `{Array<number>}` data - **Đầu vào:** Một mảng các giá trị số để vẽ đường kẻ.
 * @param `{object}` [options] - **Đầu vào:** (Tùy chọn) Đối tượng cấu hình bổ sung.
 * @param `{string}` [options.color] - Màu của đường kẻ và vùng nền. Mặc định là 'var(--accent-color)'.
 * @param `{string}` [options.valuePrefix] - Tiền tố thêm vào trước mỗi giá trị nhãn (vd: '$').
 * @param `{string}` [options.valueSuffix] - Hậu tố thêm vào sau mỗi giá trị nhãn (vd: '%').
 * @returns `{void}` **Đầu ra:** Hàm này không trả về giá trị. Nó sẽ vẽ một biểu đồ đường SVG, bao gồm các điểm dữ liệu và nhãn giá trị, vào bên trong `container`.
 
`function createLineChart(container, data, options = {}) { /* ... */ }`

---
#### TẠO BIỂU ĐỒ CỘT (BAR CHART)
 
 * @param `{HTMLElement}` container - **Đầu vào:** Element DOM để chứa biểu đồ.
 * @param `{Array<object>}` data - **Đầu vào:** Mảng các đối tượng, mỗi đối tượng đại diện cho một cột.
 * - `{number}` value: Giá trị (chiều cao) của cột.
 * - `{string}` label: Nhãn hiển thị bên dưới cột.
 * - `{string}` [color] - (Tùy chọn) Màu của cột.
 * @param {object} [options] - **Đầu vào:** (Tùy chọn) Đối tượng cấu hình bổ sung.
 * @param {string} [options.valuePrefix] - Tiền tố thêm vào trước mỗi giá trị trên cột (vd: '$').
 * @param {string} [options.valueSuffix] - Hậu tố thêm vào sau mỗi giá trị trên cột (vd: 'B').
 * @param {string} [options.yAxisLabel] - Nhãn cho trục Y (vd: 'Tỷ USD').
 * @returns `{void}` **Đầu ra:** Hàm này không trả về giá trị. Nó sẽ vẽ một biểu đồ cột SVG, bao gồm nhãn trục Y và các giá trị được định dạng, vào bên trong `container`.

`function createBarChart(container, data, options = {}) { /* ... */ }`

---
#### TẠO BIỂU ĐỒ TRÒN (DOUGHNUT CHART)

 * @param `{HTMLElement}` container - **Đầu vào:** Element DOM để chứa biểu đồ.
 * @param `{Array<object>}` data - **Đầu vào:** Mảng các đối tượng, mỗi đối tượng đại diện cho một phần của biểu đồ.
 * - `{number}` value: Giá trị của phần đó, dùng để tính tỷ lệ.
 * - `{string}` color: Màu của phần đó.
 * - `{string}` label: Nhãn văn bản cho phần đó, sẽ được hiển thị trong chú giải.
 * @param `{object|string}` config - **Đầu vào:** Đối tượng cấu hình cho biểu đồ hoặc title string (backward compatibility).
 * @param `{string}` [config.title=''] - (Tùy chọn) Tiêu đề để hiển thị ở giữa biểu đồ. Ví dụ: BTC.D
 * @param `{number}` [config.outerRadius=80] - (Tùy chọn) Bán kính ngoài của biểu đồ.
 * @param `{number}` [config.innerRadius=50] - (Tùy chọn) Bán kính trong của biểu đồ.
 * @param `{boolean}` [config.showLegend=true] - (Tùy chọn) Có hiển thị chú thích hay không.
 * @returns `{void}` **Đầu ra:** Hàm này không trả về giá trị.
 * Nó sẽ vẽ một biểu đồ doughnut SVG với tiêu đề ở giữa và một phần chú giải chi tiết vào trong `container`.
 * Chú ý: chọn màu tương phản nhau cho các đối tượng khác nhau
 * 
 * **Ví dụ sử dụng:**
 * ```javascript
 * createDoughnutChart(container, data, {
 *     title: 'BTC.D',
 *     showLegend: true,
 *     outerRadius: 80,
 *     innerRadius: 50
 * });
 * 
 * ```
`function createDoughnutChart(container, data, config = {}) { /* ... */ }`

#### Ví dụ:
```javascript
function initializeAllVisuals_report() {
  
    // Gọi tất cả biểu đồ ở đây, truyền lang xuống các hàm con
    initializeBTCDominance_report();
    initializeFearGreedGauge_report();
}

function initializeBTCDominance_report() {
    const id = 'btc-dominance-doughnut-container';
    const container = document.getElementById(id);
    if (!container) return;
    const data = [
        {value: 52.5, color: '#f7931a', label: 'Bitcoin'},
        {value: 47.5, color: '#627eea', label: 'Altcoins'}
    ];
    
    const config = {
        title: 'BTC.D',
        showLegend: true,
        outerRadius: 80,
        innerRadius: 50
    };
    
    
    createDoughnutChart(container, data, config);
}

function initializeFearGreedGauge_report() {
    const id  = 'fear-greed-gauge-container';
    const container = document.getElementById(id);
    if (!container) return;

    const value = 45;
    const config = {
    min: 0,
    max: 100,
    segments: [
        {limit: 25, color: '#ff0000', label: 'Cực kỳ sợ hãi'},
        {limit: 45, color: '#ff4500', label: 'Sợ'},
        {limit: 75, color: '#32cd32', label: 'Tham lam'},
        {limit: 100, color: '#006400', label: 'Cực kỳ tham lam'}
    ]
    };
    createGauge(container, value, config);
}
```

### Làm đúng theo quy ước và ví dụ đã cho"#;
