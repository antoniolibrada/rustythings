import {
  h,
  Component,
  render,
} from "https://cdn.jsdelivr.net/npm/preact/dist/preact.mjs";

class App extends Component {
  constructor() {
      super();
      this.state = {
          todos: []
      };
  }

  getData() {
      return fetch("/todo/")
          .then((res) => res.json())
          .then((data) => this.setState({
              todos: data
          }));
  }
  componentDidMount() {
      return this.getData()
  }
  send(e) {
      e.preventDefault();
      const formData = new FormData(e.currentTarget);
      return fetch("/todo/", {
              method: "POST",
              headers: {
                  "Content-Type": "application/json"
              },
              body: JSON.stringify({
                  title: formData.get("title")
              })
          })
          .then(() => this.getData())
  }

  render() {
      return h("div", {},
          h("form", {
                  onSubmit: e=> this.send(e)
              },
              h("input", {
                  type: "text",
                  id: "title",
                  placeholder: "What do you have to do",
                  name: "title"
              }),
              h("button", {}, "Add")
          ),
          h(
              "ul", {},
              this.state.todos.map((todo) => h("li", {}, todo.title))
          ));
  }
}

render(h(App), document.body, document.body.lastChild);