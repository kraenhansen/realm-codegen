#include "jsi/jsi.h"
#include <NapiJsiRuntime.h>
#include <napi.h>

class ExampleAddon : public Napi::Addon<ExampleAddon> {
public:
  ExampleAddon(Napi::Env env, Napi::Object exports) {
    DefineAddon(exports, {InstanceMethod("register", &ExampleAddon::Register)});
  }

private:
  Napi::Value Register(const Napi::CallbackInfo &info) {
    using namespace facebook;
    // Setup a jsi runtime
    auto runtime = Microsoft::JSI::MakeNapiJsiRuntime(info.Env());
    auto obj = jsi::Object(*runtime);
    obj.setProperty(*runtime, "foo", "bar");

    // Expose the object as a global, read it as a NAPI value and return it
    runtime->global().setProperty(*runtime, "addonExport", obj);
    auto napi_global = info.Env().Global();
    auto napi_obj = napi_global.Get("addonExport");
    napi_global.Delete("addonExport");
    return napi_obj;
  }
};

// The macro announces that instances of the class `ExampleAddon` will be
// created for each instance of the add-on that must be loaded into Node.js.
NODE_API_ADDON(ExampleAddon)