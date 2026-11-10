// Backend de demonstração em C++: um campo, um botão e a resposta do backend.
//
//   cl /std:c++17 /EHsc /utf-8 /O2 clients\cpp\demo.cpp /Fe:clients\cpp\demo.exe
//   g++ -std=c++17 -O2 clients/cpp/demo.cpp -o clients/cpp/demo
//   aylis-host -- clients/cpp/demo.exe
//   aylis-host --png tela.png -- clients/cpp/demo.exe --selftest

#include <fstream>
#include <sstream>
#include <string>

#include "aylis.hpp"

int main(int argc, char** argv) {
    bool selftest = argc > 1 && std::string(argv[1]) == "--selftest";
    aylis::Aylis app;

    std::ifstream file("clients/demo_layout.json", std::ios::binary);
    std::stringstream text;
    text << file.rdbuf();
    app.send("load", {{"layout", aylis::Json::parse(text.str())}});

    app.on("hello", [&](const aylis::Event&) {
        std::string name = app.call("get", {{"target", "edtName"}, {"prop", "text"}}).str();
        if (name.empty()) name = "mundo";
        app.send("set", {{"target", "lblMsg"}, {"props", {{"text", "Olá, " + name + "! (C++)"}}}});
        if (selftest) {
            aylis::log("SELFTEST cpp " + app.call("get", {{"target", "lblMsg"}, {"prop", "text"}}).str());
            // Fechar pelo backend passa pelo on_close_query, que pergunta (e espera a resposta).
            app.send("close", {{"target", "frmDemo"}});
        }
    });

    // O on_close_query pergunta: a tela só fecha com a resposta (o ev.reply).
    app.on("pode_fechar", [](const aylis::Event& ev) {
        aylis::log("SELFTEST cpp pergunta");
        aylis::log("SELFTEST cpp " + aylis::Delta::selftest());
        ev.reply.set("can_close", true);
    });

    app.call("run", {{"form", "frmDemo"}});
    if (selftest) {
        app.send("set", {{"target", "edtName"}, {"props", {{"text", "Ana"}}}});
        app.send("click", {{"target", "btnHello"}});
    }
    app.loop();
    return 0;
}
