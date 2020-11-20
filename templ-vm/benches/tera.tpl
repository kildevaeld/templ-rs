{{greeting | capitalize}}, {{subject}} {{age+1}}!
{% if age > 100 %}
You are old
{% elif age == 100 %}
You are 100 years old
{% else %}
You are {{age}} 
{% endif %}