


{% template Layout(title string) %}
<html>
    <head>
        <title>{{title}}</title>
        {% block head %}
    </head>
    <body>
        {% block main %}
        Default, baby!
        {% endblock %}
    </body>
</html>
{% end %}



{% template Index(title string) extends Layout %}

{% block main %}
<h1>Hello, World!</h1>
{% endblock %}

{% end %}