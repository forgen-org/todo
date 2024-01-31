@file:OptIn(ExperimentalMaterial3Api::class)

package tech.forgen.todolist

import android.annotation.SuppressLint
import android.widget.Toast
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.text.BasicTextField
import androidx.compose.material.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.ExperimentalComposeUiApi
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalSoftwareKeyboardController
import androidx.compose.ui.text.input.TextFieldValue
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.launch
import uniffi.mobile.*

@OptIn(ExperimentalComposeUiApi::class)
@SuppressLint("UnusedMaterial3ScaffoldPaddingParameter")
@Composable
fun TodoList() {
    var todolist by remember { mutableStateOf<TodoListDto?>(null) }
    var newTaskName by remember { mutableStateOf(TextFieldValue("")) }

    val context = LocalContext.current
    val client = remember { Client() }
    val coroutineScope = rememberCoroutineScope()
    val keyboardController = LocalSoftwareKeyboardController.current

    fun handleError(error: Throwable) {
        val errorMessage = when (error) {
            is ErrorDto.Exception -> {
                error.description
            }
            else -> {
                error.message ?: "An unknown error occurred"
            }
        }
        Toast.makeText(context, errorMessage, Toast.LENGTH_SHORT).show()
    }

    fun completeTask(taskIndex: UInt) {
        coroutineScope.launch {
            try {
                client.completeTask(taskIndex)
                todolist = client.getTodoList()
            } catch (e: Exception) {
                handleError(e)
            }
        }
    }

    fun addTask() {
        keyboardController?.hide()
        
        coroutineScope.launch {
            try {
                client.addTask(newTaskName.text)
                todolist = client.getTodoList()
                newTaskName = TextFieldValue("")
            } catch (e: Exception) {
                handleError(e)
            }
        }
    }

    Scaffold(
        topBar = {
            TopAppBar(title = { Text("TodoList") })
        },
        content = {
            Box {
                Column(
                    modifier = Modifier
                        .padding(16.dp)
                        .fillMaxSize()
                ) {
                    todolist?.tasks?.let { tasks ->
                        tasks.forEach { task ->
                            Row(
                                modifier = Modifier.fillMaxWidth(),
                                verticalAlignment = Alignment.CenterVertically
                            ) {
                                Text(text = task.name, modifier = Modifier.weight(1f))

                                Switch(
                                    checked = task.status == TaskStatusDto.COMPLETED,
                                    onCheckedChange = {
                                        if (task.status != TaskStatusDto.COMPLETED) completeTask(
                                            task.index
                                        )
                                    }
                                )
                            }
                        }
                    }
                }

                // Input and Button at the bottom
                Column(modifier = Modifier
                    .align(Alignment.BottomCenter)
                    .fillMaxWidth()
                    .padding(16.dp)) {
                    Row(
                        modifier = Modifier.imePadding(),
                        verticalAlignment = Alignment.CenterVertically
                    ) {
                        BasicTextField(
                            value = newTaskName,
                            onValueChange = { newTaskName = it },
                            modifier = Modifier.weight(1f).padding(end = 8.dp)
                        )

                        Button(
                            modifier = Modifier,
                            onClick = { addTask() }
                        ) {
                            Text("Add")
                        }
                    }
                }
            }
        },
    )
}
