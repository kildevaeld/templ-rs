{% template WhitespaceControl(test bool) %}
{% if test -%}
Rapper
{%- endif -%}
{% end %}