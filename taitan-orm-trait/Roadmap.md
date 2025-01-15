1. 轻量化，把不应该在trait中的部分移动到macro中去 
（1）parser基本全部需要移动到macro
2. Error可能需要重新设计，taitan-orm中直接使用这里的error就够了
3. 测试全面覆盖