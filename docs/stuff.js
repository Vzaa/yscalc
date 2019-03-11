Vue.component('person-entry', {
  props: ['name', 'id'],
  template: `
    <button
    class="btn btn-outline-info"
    @click="del_name(id)">
      {{ name }}
      <span class="badge badge-danger">x</span>
    </button>
  `,
  methods: {
    del_name: function(id) {
      this.$emit('delet-person', id)
    }
  }
})

Vue.component('result-entry', {
  props: ['res', 'id'],
  template: `
    <tr>
      <td>
      {{ res.name }}
      </td>
      <td>
      {{ res.total.toFixed(2) }} ({{ res.total_nr.toFixed(2) }})
      </td>
      <td>
      {{res.sum().toFixed(2)}} ({{ res.item_str() }})
      </td>
    </tr>
  `,
  methods: {
  }
})

Vue.component('item-entry', {
  props: ['price', 'shared', 'id', 'persons'],
  template: `
  <tr>
    <td scope="col" width="5%">
      {{ id + 1 }}
    </td>
    <td scope="col" width="5%">
      <span class="badge badge-primary">{{ price.toFixed(2) }}</span>
    </td>
    <td scope="col" width="25%">
      <select class="custom-select" @change="add_shared(id, $event)">
        <option disabled selected value="">Add</option>
        <option v-for="p in persons">{{ p }}</option>
      </select>
    </td>
    <td scope="col">
      <span class="badge badge-secondary" v-if="shared.length == 0">Everyone</span>
      <button
        class="btn btn-sm btn-outline-info"
        @click="del_shared(id, name)"
        v-for="name in shared"
        >
        {{ name }}
      <span class="badge badge-danger">x</span>
        </button>
    </td>
    <td scope="col" width="5%">
      <button class="btn btn-danger" @click="del_item(id)">
        Del
      </button>
    </td>
  </tr>
  `,
  methods: {
    add_shared: function(id, e) {
      this.$emit('add-shared', id, e.target.value)
      e.target.selectedIndex = 0
    },
    del_shared: function(id, name) {
      this.$emit('delet-shared', id, name)
    },
    del_item: function(id) {
      this.$emit('delet-item', id)
    }
  },
})

var app = new Vue({
                  el: '#app',
                  data: {
                    persons: [],
                    items: [],
                    joker: true,
                    debug: false
                  },
                  methods: {
                    add_person: function() {
                      var name = this.$refs.name_input.value

                      if (name.trim() == "") {
                        return
                      }

                      if (this.persons.indexOf(name) == -1) {
                        this.persons.push(name.trim())
                      }
                      this.$refs.name_input.value = ""
                    },
                    add_item: function() {
                      var price = Number(this.$refs.item_input.value)
                      if (price <= 0 ) {
                        this.$refs.item_input.value = ""
                        return
                      }
                      var item = { price: price, shared: [] }
                      this.items.push(item)
                      this.$refs.item_input.value = ""
                    },
                    del_person: function(id) {
                      var name = this.persons[id]
                      this.persons.splice(id, 1)

                      if (this.persons.length == 0) {
                        this.items = []
                      } else {
                        this.items.forEach(function(item) {
                          item.shared = item.shared.filter(n => n !== name)
                        })
                      }
                    },
                    del_item: function(id) {
                      this.items.splice(id, 1)
                    },
                    add_shared: function(id, name) {
                      if (this.items[id].shared.indexOf(name) == -1) {
                        this.items[id].shared.push(name)
                      }
                    },
                    del_shared: function(id, name) {
                      this.items[id].shared = this.items[id].shared.filter(n => n !== name)
                    },
                  },
                  computed: {
                    calc: function() {
                      return testan(this.persons, this.items, this.joker).sort(function (a, b) {
                        if (a.name < b.name)
                          return -1;
                        if (a.name > b.name)
                          return 1;
                        return 0;
                      })
                    },
                    paid: function() {
                      var paid = this.calc.map(function (x) { return x.total; }).reduce(function (a, b) { return a + b; }, 0)
                      return paid
                    },
                    org_before: function() {
                      var paid = this.items.map(function (x) { return x.price; }).reduce(function (a, b) { return a + b; }, 0)
                      return paid
                    },
                    org_sum: function() {
                      let price;
                      if (this.joker) {
                        price = this.org_before - price_cut(this.org_before);
                      } else {
                        price = this.org_before;
                      }
                      return price
                    },
                    tip: function() {
                      return (this.paid - this.org_sum).toFixed(2)
                    },
                    ratio: function() {
                      return (this.paid / this.org_before).toFixed(2)
                    },
                    to_json: function() {
                      var as_arr = [];

                      this.items.forEach(function(item) {
                        if (item.shared.length == 0) {
                          as_arr.push([item.price, null])
                        } else {
                          as_arr.push([item.price, item.shared])
                        }
                      })

                      //return JSON.stringify(as_arr, null, 2)
                      return JSON.stringify(as_arr)
                    }
                  }

})
