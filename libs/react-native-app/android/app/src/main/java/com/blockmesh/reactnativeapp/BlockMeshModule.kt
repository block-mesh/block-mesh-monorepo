package com.blockmesh.reactnativeapp

import com.facebook.react.bridge.ReactApplicationContext
import com.facebook.react.bridge.ReactContextBaseJavaModule
import com.facebook.react.bridge.ReactMethod
import com.facebook.react.bridge.Callback

class BlockMeshModule(reactContext: ReactApplicationContext) : ReactContextBaseJavaModule(reactContext) {

    init {
        // Load the shared library
        System.loadLibrary("blockmesh_cli")
    }

    override fun getName(): String {
        return "BlockMeshModule"
    }

    // Declare native method
    external fun runLib(email: String, password: String)

    @ReactMethod
    fun runBlockmeshLib(email: String, password: String, callback: Callback) {
        try {
            runLib(email, password)
            callback.invoke(null, "Success")
        } catch (e: Exception) {
            callback.invoke(e.message)
        }
    }
}