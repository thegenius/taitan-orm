
现在，生成struct和trait impl是混合的，应该分开为struct实现和impl


目前正在进行的工作：
1. 支持手写SelectedEntity，支持字段不是optional
2. 支持TemplateLocation，必需绑定SelectedEntity
3. Schema中支持index定义，并自动生成查询函数
4. Schema中支持自定义query，通过SelectedEntity做Selection和location参数
5. 支持template中通过 %{} 语法识别Optional字段