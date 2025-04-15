# 學生座位分配工具
透過將 excel 和 word 檔案進行連結以實現座位亂數分配。

## 功能
- 透過 excel 檔案輸入學生名稱
- 透過 word 檔案生成座位表
- 亂數分配座位
- 自定義參數

## 安裝

## 初次使用設定
<details>
  <summary><strong>Click to expand</strong></summary>

1. 開啟 `output\random seats.docm`，找到 `UpdateWordLinks` 巨集並編輯。

    ![Macro](https://iili.io/30HypBn.png)

2. 將巨集中的 newFilePath 變數修改為你本地的 `input\student_data.xlsx` 絕對路徑，oldFilePath 變數一般不需要更改。
    ```
    oldFilePath = "C:\Users\ASUS\Code\random_seats\input\student_data.xlsx"

    newFilePath = "C:\Users\ASUS\Code\Rust\random_seats\input\student_data.xlsx"
    ```
</details>

## 使用方法
<details>
  <summary><strong>Click to expand</strong></summary>

1. 打開 `input\student_data.xlsx` 檔案，切換到 student 工作表，將學生名稱從 A1 開始往下輸入，輸入完畢記得保存，然後退出。

    ![student_data](https://iili.io/30JVfCx.png)

2. 啟動 random_seats.exe，有一些能自定義的參數，參數修改好後按下 Start 按鈕，等待生成完成。

    | 參數 | 描述 |
    |-------|-------|
    | `Input path` | `student_data.xlsx` 的路徑，通常不用修改 |
    | `Max Seats` | 最大座位數量 |
    | `排除座位` | 不能被分配的座位編號 |

    ![exe](https://iili.io/30J4E0P.png)

3. 生成完成後，打開 `output\random seats.docm`，即可看到生成的座位表。

    ![random seats](https://iili.io/30JtuN1.png)

</details>
