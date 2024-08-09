package xyz.blockmesh
import android.os.Handler
import androidx.annotation.NonNull
import com.facebook.react.bridge.ReactApplicationContext
import com.facebook.react.bridge.ReactContextBaseJavaModule
import com.facebook.react.bridge.ReactMethod
import com.facebook.react.bridge.Promise

class RustBridgeModule(reactContext: ReactApplicationContext) : ReactContextBaseJavaModule(reactContext) {

    init {
        Companion.reactContext = reactContext
    }

    companion object {
        private lateinit var reactContext: ReactApplicationContext

        @JvmStatic
        private external fun runLib(url: String, email: String, password: String): Int

        init {
            System.loadLibrary("blockmesh-cli")
        }
    }

    @NonNull
    override fun getName(): String {
        return "RustModule"
    }

    @ReactMethod
    fun runLib(url: String, email: String, password: String, promise: Promise) {
        val result = runLib(url, email, password)
        promise.resolve(result)
    }
}
