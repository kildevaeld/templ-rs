
type User {
    name string
    age number
}

type BlogPageModel {
    name string
    published date
    author User
    comments []string
}

{% template Comment(idx number, body string) %}
#{{ i + 1 }} {{ v }}
{% end %}

{% template BlogPage(ctx BlogPageModel) %}
Hello {{ ctx.name | capitalize() | uppercase }}, age: {{ ctx.age + 2 }}
{# Comment 
   Here
#}

Comments:
{% for i, v in ctx.comments %}
  {% include Comment(i, v) %}
    {{ ctx.name }}
  {% endinclude %}
  
{% endfor %}

{% if ctx.name == "Rasmus" %}
  
{% elif ctx.name == "Feja" %}

{% else %}

{% endif %}
{% end %}