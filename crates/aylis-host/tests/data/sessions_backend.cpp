// Backend de teste das sessões em C++ (aylis-host --bridge ... --sessions): um processo para
// todas as telas, um contador por tela; "boom" lança uma exceção que fica só naquela sessão,
// "lento" demora sem travar as outras e "entrar"/"comum" ligam as telas do mesmo usuário.

#include <chrono>
#include <thread>

#include "../../../../clients/cpp/aylis.hpp"

#ifdef _WIN32
#include <process.h>
#define getpid _getpid
#else
#include <unistd.h>
#endif

using aylis::Json;

static Json button(const char* name, const char* handler) {
    return Json::object({{"class", Json("Button")}, {"name", Json(name)}, {"text", Json(name)}, {"on_click", Json(handler)}});
}

static void set_text(aylis::Session& s, const std::string& target, const std::string& text) {
    s.send("set", Json::object({{"target", Json(target)}, {"props", Json::object({{"text", Json(text)}})}}));
}

int main() {
    Json layout = Json::object({{"class", Json("Form")}, {"name", Json("frm")}, {"width", Json(200)}, {"height", Json(80)},
        {"children", Json::array({Json::object({{"class", Json("Label")}, {"name", Json("lbl")}, {"text", Json("-")}}), button("btn", "somar"),
            button("boom", "boom"), button("lento", "lento"), Json::object({{"class", Json("Label")}, {"name", Json("lblu")}, {"text", Json("-")}}),
            button("entrar", "entrar"), button("comum", "comum")})}});
    aylis::Server::serve(
        [&](aylis::Session& s) {
            int total = 0;
            aylis::log("SELFTEST sessão " + std::to_string(s.number()) + " abriu (pid " + std::to_string(getpid()) + ")");
            s.on("somar", [&](const aylis::Event&) { set_text(s, "frm.lbl", std::to_string(s.number()) + ":" + std::to_string(++total)); });
            s.on("boom", [](const aylis::Event&) { throw std::runtime_error("erro de propósito"); });
            s.on("lento", [](const aylis::Event&) { std::this_thread::sleep_for(std::chrono::seconds(3)); });
            s.on("entrar", [&](const aylis::Event&) {
                s.login("ana");
                set_text(s, "frm.lblu", "ana");
            });
            s.on("comum", [&](const aylis::Event&) {
                // O profile (a faixa "Marcas" do profiler): confere que compila e roda.
                auto mark = s.profile("comum");
                auto u = s.user();
                int n;
                {
                    std::lock_guard<std::mutex> g(u->mutex);
                    n = static_cast<int>(u->data["total"].number()) + 1;
                    u->data["total"] = Json(n);
                }
                u->broadcast([n](aylis::Session& t) { set_text(t, "frm.lblu", "u:" + std::to_string(n)); });
            });
            s.send("load", Json::object({{"layout", layout}}));
            s.run("frm");
        },
        [](aylis::Session& s) { aylis::log("SELFTEST sessão " + std::to_string(s.number()) + " saiu"); });
}
