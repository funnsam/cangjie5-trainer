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

    let corr_c = window.localStorage.getItem("corr") | 0;
    let totl_c = window.localStorage.getItem("totl") | 0;

    function rescramble() {
        n = Math.floor(Math.random() * chars.length);
        target.innerText = chars[n][0];
        set_chi_class(target);
    }
    rescramble();

    input.onkeydown = (e) => {
        let is_valid = e.key.length == 1 && 0x61 <= e.key.codePointAt(0) && e.key.codePointAt(0) <= 0x79;

        if (e.key == " ") {
            e.preventDefault();

            if (ans_shown) {
                e.target.value = "";
                rescramble();
                answer.innerText = "";
            } else {
                let code = chars[n][1];
                let code_txt = code.join(" / ");

                answer.innerText = code_txt;
                let corr = code.includes(e.target.value)

                if (corr) answer.classList.add("correct");
                else answer.classList.remove("correct");

                corr_c += corr;
                totl_c += 1;
                window.localStorage.setItem("corr", corr_c);
                window.localStorage.setItem("totl", totl_c);

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
                    const ans = document.createTextNode(code_txt);
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
