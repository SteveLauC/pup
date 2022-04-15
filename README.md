### pup
A command line tool aiming to upload the local iamge used in your markdown file to
the GitHub repo and replace the local file path with returned URL.


### How it works
![workflow](https://github.com/SteveLauC/pic/blob/main/Page%201.png)

### How to use
```shell
on localhost pup  🍣 main [📝🤷✓] is 📦 v0.1.0 via 🦀 v1.60.0
❯ ./target/debug/pup test.md
find: "/Users/steve/Desktop/pic1.png"
DONE
find: "/Users/steve/Desktop/pic2.png"
DONE
find: "/Users/steve/Desktop/pic3.png"
DONE

on localhost pup  🍣 main [📝🤷✓] is 📦 v0.1.0 via 🦀 v1.60.0 took 8s
❯ bat test.md
───────┬─────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
       │ File: test.md
       │ Size: 3.6 KB
───────┼─────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
   1   │ 1. 由于浮点数在运算的时候会不断的丢失精度，所以我们在给浮点数加上附加位来留住精度。
   2   │    IEEE754规定，必须在中间结果的右边加两个附加位(guard and round)
   3   │
   4   │    ![illustration](https://github.com/SteveLauC/test-repo/blob/main/pic1.png)
   5   │
   6   │    这几个位的默认值是0，在运算或者对阶移动或者非规格化尾数或者尾数溢出的时候就会发
   7   │    挥作用。比如在右规时可以用来保护位，左规时被移动到mantissa中，作为舍入的依据。
   8   │
   9   │ 2. 举个十进制小数的例子来说明附加位对精度的影响
  10   │
  11   │    ![illustration](https://github.com/SteveLauC/test-repo/blob/main/pic2.png)
  12   │
  13   │
  14   │ 3. IEEE754的4种舍入模型
  15   │
  16   │    The IEEE standard has four different rounding modes; the first is the default;
  17   │    the others are called directed roundings.
  18   │
  19   │    1. Round to Nearest – rounds to the nearest value;
  20   │    > if the number falls midway it is rounded to the nearest value with an
  21   │    *even* (zero) least significant bit, which means it is rounded up 50% of the
  22   │    time (in IEEE 754-2008 this mode is called roundTiesToEven to distinguish it
  23   │    from another round-to-nearest mode)
  24   │     ![ppt](https://github.com/SteveLauC/pic/blob/main/Screen%20Shot%202022-03-28%20at%2010.31.38%20AM.png)
```