var Item = /** @class */ (function () {
    function Item() {
    }
    return Item;
}());
var Entry = /** @class */ (function () {
    function Entry(n, i) {
        this.name = n;
        this.items = i;
        this.total = 0.0;
    }
    Entry.prototype.push = function (v) {
        this.items.push(v);
    };
    Entry.prototype.sum = function () {
        return this.items.reduce(function (a, b) { return a + b; }, 0);
    };
    Entry.prototype.item_str = function () {
        return this.items.map(function (a) { return a.toFixed(2); }).join(', ');
    };
    return Entry;
}());
function round4(tot) {
    return Math.round(tot * 4.0) / 4.0;
}
function ceil4(tot) {
    return Math.ceil(tot * 4.0 - 0.00001) / 4.0;
}
function price_cut(tot) {
    if (tot < 30.0) {
        return 0.0;
    }
    else if (tot < 40.0) {
        return 30.0 - 20.0;
    }
    else if (tot < 70.0) {
        return 40.0 - 25.0;
    }
    else if (tot < 120.0) {
        return 70.0 - 45.0;
    }
    else {
        return 120.0 - 75.0;
    }
}
function yscalc(entries, joker) {
    var sum_before = entries.map(function (x) { return x.sum(); }).reduce(function (a, b) { return a + b; }, 0);
    var sum_after = sum_before;
    if (joker) {
        sum_after -= price_cut(sum_before);
    }
    var ratio = sum_after / sum_before;
    var pay = entries.map(function (e) { return e.sum() * ratio; });
    var pay_rounded = pay.map(function (c) { return round4(c); });
    var rounded_sum = pay_rounded.reduce(function (a, b) { return a + b; }, 0);
    var remainder = ceil4(sum_after) - rounded_sum;
    while (true) {
        var min_idx = pay_rounded.map(function (e, i) { return [e - pay[i], i]; })
            .reduce(function (a, b) { if (a[0] < b[0]) {
            return a;
        }
        else {
            return b;
        } })[1];
        var max_idx = pay_rounded.map(function (e, i) { return [e - pay[i], i]; })
            .reduce(function (a, b) { if (a[0] > b[0]) {
            return a;
        }
        else {
            return b;
        } })[1];
        if (remainder > 0.0) {
            if (Math.abs(remainder) >= 0.25) {
                pay_rounded[min_idx] += 0.25;
                remainder -= 0.25;
            }
            else {
                pay_rounded[min_idx] += remainder;
                remainder = 0.0;
                break;
            }
        }
        else {
            if (Math.abs(remainder) >= 0.25) {
                pay_rounded[max_idx] -= 0.25;
                remainder += 0.25;
            }
            else {
                // tipping the remainder
                break;
            }
        }
    }
    for (var i = 0; i < pay_rounded.length; i++) {
        entries[i].total = pay_rounded[i];
        entries[i].total_nr = pay[i];
    }
    return entries;
}
function testan(names, items, joker) {
    var dat = new Map();
    for (var _i = 0, names_1 = names; _i < names_1.length; _i++) {
        var name = names_1[_i];
        dat.set(name, new Entry(name, []));
    }
    if (items.length == 0) {
        return [];
    }
    for (var _a = 0, items_1 = items; _a < items_1.length; _a++) {
        var item = items_1[_a];
        var v_div = void 0;
        if (item.shared.length == 0) {
            v_div = item.price / names.length;
            for (var _b = 0, names_2 = names; _b < names_2.length; _b++) {
                var name = names_2[_b];
                dat.get(name).push(v_div);
            }
        }
        else {
            v_div = item.price / item.shared.length;
            for (var _c = 0, _d = item.shared; _c < _d.length; _c++) {
                var name = _d[_c];
                dat.get(name).push(v_div);
            }
        }
    }
    var as_list = [];
    dat.forEach(function (value, _) {
        if (value.items.length > 0) {
            as_list.push(value);
        }
    });
    return yscalc(as_list, joker);
}
