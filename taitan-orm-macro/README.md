
现在，生成struct和trait impl是混合的，应该分开为struct实现和impl


目前正在进行的工作：
1. 支持手写SelectedEntity，支持字段不是Optional: Done
2. 支持template中通过 %{} 语法识别Optional字段

3. 支持TemplateLocation，实现了Location trait, 必需绑定SelectedEntity
   这个比TemplateRecord多的就是可以筛选字段


4. Schema中支持index定义，并自动生成查询函数
5. Schema中支持自定义query，通过SelectedEntity做Selection和location参数
