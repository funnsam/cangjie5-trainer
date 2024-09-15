import init, * as cangjie5 from "./pkg/cangjie5_trainer_web.js";

function set_chi_class(t) {
    t.classList.remove("chi_plane0");
    t.classList.remove("chi_plane2");
    t.classList.remove("chi_plane3");
    t.classList.add("chi_plane" + (t.innerText.codePointAt(0) >> 16));
}

document.addEventListener("DOMContentLoaded", (_) => {
    const target = document.getElementById("target");
    const input = document.getElementById("input");
    const answer = document.getElementById("answer");
    const stats = document.getElementById("stats");
    const errors = document.getElementById("errors");

    input.value = "";

    // gamestate
    let n = 0;
    let ans_shown = false;

    let corr_c = 0;
    let totl_c = 0;

    function rescramble() {
        n = Math.floor(Math.random() * cangjie5.chars_len());
        target.innerText = cangjie5.id_char(n);
        set_chi_class(target);
    }
    init().then(rescramble);

    input.onkeydown = (e) => {
        let is_valid = e.key.length == 1 && 0x61 <= e.key.codePointAt(0) && e.key.codePointAt(0) <= 0x79;

        if (e.key == " ") {
            e.preventDefault();

            if (ans_shown) {
                e.target.value = "";
                rescramble();
                answer.innerText = "";
            } else {
                answer.innerText = cangjie5.id_codes(n).join(" / ");
                let corr = cangjie5.id_codes(n).includes(e.target.value)

                if (corr) answer.classList.add("correct");
                else answer.classList.remove("correct");

                corr_c += corr;
                totl_c += 1;

                stats.innerText = `${corr_c} / ${totl_c} (${(corr_c / totl_c * 100).toFixed(1)}%)`;

                if (!corr) {
                    const line = document.createElement("div");

                    const text_div = document.createElement("div");
                    line.appendChild(text_div);
                    text_div.classList.add("text");
                    const text = document.createTextNode(target.innerText);
                    text_div.appendChild(text);
                    set_chi_class(text_div);

                    const incor_div = document.createElement("div");
                    line.appendChild(incor_div);
                    incor_div.classList.add("may_correct");
                    const incor = document.createTextNode(e.target.value.padEnd(5, " "));
                    incor_div.appendChild(incor);

                    const ans_div = document.createElement("div");
                    line.appendChild(ans_div);
                    ans_div.classList.add("may_correct");
                    ans_div.classList.add("correct");
                    const ans = document.createTextNode(answer.innerText);
                    ans_div.appendChild(ans);

                    errors.prepend(line);
                }
            }

            ans_shown ^= true;
        } else if (ans_shown || (is_valid && e.target.value.length >= 5) || (!is_valid && e.key != "Backspace")) {
            e.preventDefault();
        }
    };
});
