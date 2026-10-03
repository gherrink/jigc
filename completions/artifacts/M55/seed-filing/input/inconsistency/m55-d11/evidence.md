```sh
sed -n '/^The flows:/,/^## 1\./p' design/worked-examples.md | grep -c '^[0-9]*\. \['   # 16
grep -c '^## [0-9]*\.' design/worked-examples.md                                     # 54
```
