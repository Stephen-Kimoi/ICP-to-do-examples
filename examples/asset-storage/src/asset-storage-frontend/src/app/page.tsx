'use client'

import { useState } from 'react'
import { asset_storage_backend } from '../../../declarations/asset_storage_backend'
import { FileUpload } from '@/components/file-upload'
import { ImageGrid } from '@/components/image-grid'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { toast } from '@/components/ui/use-toast'

export default function Home() {
  const [images, setImages] = useState([])
  const [fileId, setFileId] = useState(null)
  const [isLoadingImage, setIsLoadingImage] = useState(false)
  const [isLoadingFile, setIsLoadingFile] = useState(false)

  const handleImageUpload = async (file: File) => {
    setIsLoadingImage(true)
    const reader = new FileReader()
    
    reader.onload = async () => {
      try {
        const imageData = new Uint8Array(reader.result as ArrayBuffer)
        const chunks = Array.from(imageData)
        const id = await asset_storage_backend.upload_image([chunks])
        await loadImages()
        toast({
          title: 'Image uploaded successfully',
          description: `Image ID: ${id}`,
        })
      } catch (error) {
        console.error('Error uploading image:', error)
        toast({
          title: 'Error uploading image',
          description: 'Please try again',
          variant: 'destructive',
        })
      } finally {
        setIsLoadingImage(false)
      }
    }
    
    reader.readAsArrayBuffer(file)
  }

  const handleFileUpload = async (file: File) => {
    setIsLoadingFile(true)
    const reader = new FileReader()
    
    reader.onload = async () => {
      try {
        const fileData = new Uint8Array(reader.result as ArrayBuffer)
        const id = await asset_storage_backend.upload_file(Array.from(fileData))
        setFileId(id)
        toast({
          title: 'File uploaded successfully',
          description: `File ID: ${id}`,
        })
      } catch (error) {
        console.error('Error uploading file:', error)
        toast({
          title: 'Error uploading file',
          description: 'Please try again',
          variant: 'destructive',
        })
      } finally {
        setIsLoadingFile(false)
      }
    }
    
    reader.readAsArrayBuffer(file)
  }

  const loadImages = async () => {
    const allImages = await asset_storage_backend.get_all_images()
    setImages(allImages)
  }

  const getFile = async (id: string) => {
    const file = await asset_storage_backend.get_file(id)
    if (file) {
      const blob = new Blob([new Uint8Array(file.data)])
      const url = URL.createObjectURL(blob)
      const a = document.createElement('a')
      a.href = url
      a.download = `file-${id}`
      a.click()
    }
  }

  return (
    <main className="container mx-auto p-4">
      <h1 className="text-3xl font-bold text-center mb-8">Asset Storage</h1>
      <Tabs defaultValue="upload" className="w-full">
        <TabsList className="grid w-full grid-cols-2">
          <TabsTrigger value="upload">Upload</TabsTrigger>
          <TabsTrigger value="gallery">Gallery</TabsTrigger>
        </TabsList>
        <TabsContent value="upload">
          <div className="grid gap-8 md:grid-cols-2">
            <Card>
              <CardHeader>
                <CardTitle>Upload Image</CardTitle>
              </CardHeader>
              <CardContent>
                <FileUpload
                  accept="image/*"
                  onFileSelect={handleImageUpload}
                  isLoading={isLoadingImage}
                  buttonText="Upload Image"
                />
              </CardContent>
            </Card>
            <Card>
              <CardHeader>
                <CardTitle>Upload File</CardTitle>
              </CardHeader>
              <CardContent>
                <FileUpload
                  onFileSelect={handleFileUpload}
                  isLoading={isLoadingFile}
                  buttonText="Upload File"
                />
                {fileId && (
                  <Button
                    onClick={() => getFile(fileId)}
                    className="w-full mt-4"
                  >
                    Download File {fileId}
                  </Button>
                )}
              </CardContent>
            </Card>
          </div>
        </TabsContent>
        <TabsContent value="gallery">
          <Card>
            <CardHeader>
              <CardTitle>Uploaded Images</CardTitle>
            </CardHeader>
            <CardContent>
              <ImageGrid images={images} />
            </CardContent>
          </Card>
        </TabsContent>
      </Tabs>
    </main>
  )
}