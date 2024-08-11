package xyz.blockmesh

import androidx.annotation.NonNull
import com.facebook.react.ReactPackage
import com.facebook.react.bridge.NativeModule
import com.facebook.react.bridge.ReactApplicationContext
import com.facebook.react.uimanager.ViewManager

class RustModulePackage : ReactPackage {

    @NonNull
    override fun createNativeModules(@NonNull reactContext: ReactApplicationContext): List<NativeModule> {
        val modules: MutableList<NativeModule> = ArrayList()
        modules.add(RustModule(reactContext))
        return modules
    }

    @NonNull
    override fun createViewManagers(@NonNull reactContext: ReactApplicationContext): List<ViewManager<*, *>> {
        return emptyList()
    }
}
